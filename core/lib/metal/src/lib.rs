extern crate proc_macro;

use std::sync::Mutex;

use lazy_static::{lazy_static};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};

struct Addr {
    pub addr: u32,
}

lazy_static! {
    static ref ADDR: Mutex<Addr> =  Mutex::from(Addr {
        addr: 0x10000004
    });
}

/// metal::teleport
/// 
/// Use this attribute macro to specify a target to be transformed into hardware by Metal
#[proc_macro_attribute]
pub fn teleport(_attr: TokenStream, item: TokenStream) -> TokenStream {
    if cfg!(test) {
        return item;
    }

    let syn::Item::Fn(func) = syn::parse_macro_input!(item as syn::Item) else {
        panic!("Expected a function");
    };

    let sig = &func.sig;
    
    let mut mappers = vec![];

    for input in &sig.inputs {
        match input {
            syn::FnArg::Typed(pat_type) => {
                let pat = &pat_type.pat.to_token_stream();
                let ty = &pat_type.ty.to_token_stream();

                let addr = format!("0x{:X}", ADDR.lock().unwrap().addr);
                let addr = syn::parse_str::<syn::Expr>(&addr).unwrap();

                let map = quote! {
                    *(#addr as *mut #ty) = #pat;
                };

                ADDR.lock().unwrap().addr += 4;

                mappers.push(map);
            },
            _ => {}
        }
    }

    let addr = format!("0x{:X}", ADDR.lock().unwrap().addr);
    let addr = syn::parse_str::<syn::Expr>(&addr).unwrap();

    let syn::ReturnType::Type(_, return_type) = sig.output.clone() else {
        panic!("Expected a return type");
    };
    let return_type = return_type.to_token_stream();

    mappers.push(quote! {
        return *(#addr as *const #return_type);
    });
    ADDR.lock().unwrap().addr += 4;

    let mappers = mappers.iter();

    let tokens = quote! {
        #sig {
            unsafe {
                #(#mappers)*
            }
        }
    };

    tokens.into()
}