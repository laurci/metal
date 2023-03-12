use std::fmt::Display;

#[derive(Debug)]
pub enum MetalError {
    FailedToLoadFile(String),
    FailedToOpenDir(String),
    FailedToParseFile(String),
    FailedToWriteFile(String),
    FnShouldHaveAtLeastOneArg(String),
    FnShouldHaveExplicitReturnType(String),
    UnhandledType(String),
    UnhandledStatement(String),
    Expected(String),
}

impl Display for MetalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetalError::FailedToLoadFile(path) => write!(f, "Failed to load file: {}", path),
            MetalError::FailedToOpenDir(path) => write!(f, "Failed to open dir: {}", path),
            MetalError::FailedToParseFile(path) => write!(f, "Failed to parse file:{}", path),
            MetalError::FailedToWriteFile(path) => write!(f, "Failed to write file: {}", path),
            MetalError::FnShouldHaveAtLeastOneArg(name) => write!(f, "Function {} should have at least one argument", name),
            MetalError::FnShouldHaveExplicitReturnType(name) => write!(f, "Function {} should have explicit return type", name),
            MetalError::UnhandledType(name) => write!(f, "Unhandled type {}", name),
            MetalError::UnhandledStatement(name) => write!(f, "Unhandled statement: {}", name),
            MetalError::Expected(name) => write!(f, "Expected {}", name),
        }
    }
}