
use crate::{MetalConfig, error::MetalError, fs};
use syn::{self, __private::ToTokens};

#[derive(Debug)]
pub enum TypeDef {
    U8,
    U16,
    U32,
    I8,
    I16,
    I32,
    F32,
}

impl TypeDef {
    pub fn from_str(s: &str) -> Option<TypeDef> {
        match s {
            "u8" => Some(TypeDef::U8),
            "u16" => Some(TypeDef::U16),
            "u32" => Some(TypeDef::U32),
            "i8" => Some(TypeDef::I8),
            "i16" => Some(TypeDef::I16),
            "i32" => Some(TypeDef::I32),
            "f32" => Some(TypeDef::F32),
            _ => None
        }
    }

    pub fn size(&self) -> usize {
        match self {
            TypeDef::U8 => 1,
            TypeDef::U16 => 2,
            TypeDef::U32 => 4,
            TypeDef::I8 => 1,
            TypeDef::I16 => 2,
            TypeDef::I32 => 4,
            TypeDef::F32 => 4,
        }
    }

    pub fn bits(&self) -> usize {
        self.size() * 8
    }
}

#[derive(Debug)]
struct FnArgDef {
    name: String,
    ty: TypeDef,
}

#[derive(Debug)]
struct FnDef {
    name: String,
    args: Vec<FnArgDef>,
    return_type: TypeDef,
    stmts: Vec<String>,
}

pub struct Processor {
    config: MetalConfig,
    fns: Vec<FnDef>,
}

impl Processor {
    pub fn new(config: MetalConfig) -> Processor {
        Processor {
            config,
            fns: vec![]
        }
    }

    fn parse_return_type(&mut self, return_type: &syn::ReturnType) -> Option<String> {
        match return_type {
            syn::ReturnType::Type(_, ty) => {
                let ty = ty.to_token_stream().to_string();
                Some(ty)
            }
            _ => None   
        }
    }

    fn process_fn(&mut self, function: &syn::ItemFn) -> Result<(), MetalError> {

        let mut args =vec![];
        for arg in function.sig.inputs.iter() {
            match arg {
                syn::FnArg::Typed(typed) => {
                    let name = typed.pat.as_ref().to_token_stream().to_string();
                    let ty = typed.ty.as_ref().to_token_stream().to_string();

                    let Some(ty) = TypeDef::from_str(&ty) else {
                        return Err(MetalError::UnhandledType(ty));
                    };

                    args.push(FnArgDef {
                        name,
                        ty,
                    });
                },
                _ => panic!("Unhandled argument type")
            }
        }

        if args.len() == 0 {
            return Err(MetalError::FnShouldHaveAtLeastOneArg(function.sig.ident.to_string()))
        }

        let return_type = function.sig.output.clone();
        let Some(return_type) = self.parse_return_type(&return_type) else {
            return Err(MetalError::FnShouldHaveExplicitReturnType(function.sig.ident.to_string()))
        };
        let Some(return_type) = TypeDef::from_str(&return_type) else {
            return Err(MetalError::UnhandledType(return_type))
        };

        let mut stmts = vec![];
        for stmt in function.block.stmts.iter() {
            match stmt {
                syn::Stmt::Local(local) => {
                    let name = local.pat.to_token_stream().to_string();
                    let expr = local.init.as_ref().unwrap().1.to_token_stream().to_string();

                    stmts.push(format!("val {} = {}", name, expr));
                }
                syn::Stmt::Expr(expr) => {
                    let expr = expr.to_token_stream().to_string();
                    stmts.push(format!("io.__ret_value := {}", expr));
                }
                _ => {
                    return Err(MetalError::UnhandledStatement(stmt.to_token_stream().to_string()))
                }
            }
        }

        self.fns.push(FnDef {
            name: function.sig.ident.to_string(),
            args,
            return_type,
            stmts
        });
        
        Ok(())
    }

    pub fn process(&mut self, path: &str, content: &str) -> Result<(), MetalError> {
        let Ok(file) = syn::parse_file(content) else {
            return Err(MetalError::FailedToParseFile(path.to_owned()))
        };

        for item in file.items {
            match item {
                syn::Item::Fn(function) => {
                    let teleport_attr = function.attrs.iter().find(|&attr| {
                        if attr.path.segments.len() == 1 {
                            let first = attr.path.segments.first().unwrap().ident.clone();
                            if first.to_string() == "teleport" {
                                return true;
                            }
                        } else if attr.path.segments.len() == 2 {
                            let first = attr.path.segments.first().unwrap().ident.clone();
                            let second =  attr.path.segments.last().unwrap().ident.clone();

                            if first.to_string() == "metal" && second.to_string() == "teleport" {
                                return true;
                            }
                        }

                        false
                    });

                    if teleport_attr.is_none() {
                        continue;
                    }
                    self.process_fn(&function)?;
                },
                _ => {}
            }
        }

        Ok(())
    }

    pub fn write_output(&mut self) -> Result<(), MetalError> {
        let mut addr: u32 = 0x10000004;

        let mut modules = String::new();
        let mut mounts = String::new();

        for func in self.fns.iter() {
            let mut module = String::from(format!("class {}() extends Component ", func.name));
            
            module.push_str("{\n    val io = new Bundle {\n");

            for arg in func.args.iter() {
                module.push_str(&format!("        val {} = in UInt({} bits)\n", arg.name, arg.ty.bits()));
            }

            module.push_str(&format!("        val __ret_value = out UInt({} bits)\n", func.return_type.bits()));

            module.push_str("    }\n\n");

            for arg in func.args.iter() {
                module.push_str(&format!("    val {} = io.{}\n", arg.name, arg.name));
            }


            for stmt in func.stmts.iter() {
                module.push_str(&format!("    {};\n", stmt));
            }

            module.push_str("\n    noIoPrefix()\n}\n");

            let mut mount = String::new();

            mount.push_str(&format!("        val {}_instance = new {}()\n", func.name, func.name));

            for arg in func.args.iter() {
                mount.push_str(&format!("        val input_reg_{}_{} = slave.createReadAndWrite(UInt({} bits), 0x{:X}L, 0)\n", func.name, arg.name, arg.ty.bits(), addr));
                mount.push_str(&format!("        {}_instance.io.{} := input_reg_{}_{}\n", func.name, arg.name, func.name, arg.name));
                addr += 0x4;
            }

            mount.push_str(&format!("        val output_reg_{}_ret = slave.createReadAndWrite(UInt({} bits), 0x{:X}L, 0)\n", func.name, func.return_type.bits(), addr));
            mount.push_str(&format!("        output_reg_{}_ret := {}_instance.io.__ret_value\n", func.name, func.name));
            addr += 0x4;

            modules.push_str(&module);
            mounts.push_str(&mount);


        }

        let output = format!(r#"
package metal.generated

import spinal.core._
import spinal.lib.bus.amba3.apb._

{mods}

object GeneratedController {{
    def installSlave(slave: Apb3SlaveFactory) = {{
{mounts}
    }}
}}
"#, mods = modules, mounts = mounts);

        fs::write_output(&self.config.output, &output)?;

        Ok(())
    }
}

pub fn should_read_file(path: &str) -> bool {
    path.ends_with(".rs")
}

pub fn should_process_file(_path: &str, content: &str) -> bool {
    content.contains("#[metal::teleport]") || content.contains("#[teleport]")
}
