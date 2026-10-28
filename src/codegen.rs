use std::fmt;

use inkwell::builder::BuilderError;
use inkwell::context::Context;
use inkwell::module::Module;

#[derive(Debug)]
pub enum CodegenError {
    BuildReturn(BuilderError),
    Verification(String),
}

impl fmt::Display for CodegenError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BuildReturn(error) => {
                write!(formatter, "Failed to build return instruction: {error}")
            }
            Self::Verification(message) => {
                write!(formatter, "LLVM module verification failed: {message}")
            }
        }
    }
}

impl std::error::Error for CodegenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BuildReturn(error) => Some(error),
            Self::Verification(_) => None,
        }
    }
}

/// Generates a verified module containing `main`, which returns an i32 zero.
/// The returned module borrows the caller's LLVM context.
pub fn generate_module<'ctx>(context: &'ctx Context) -> Result<Module<'ctx>, CodegenError> {
    let module = context.create_module("cheeku");
    let builder = context.create_builder();
    let int_type = context.i32_type();
    let main = module.add_function("main", int_type.fn_type(&[], false), None);
    let entry = context.append_basic_block(main, "entry");
    builder.position_at_end(entry);

    builder
        .build_return(Some(&int_type.const_zero()))
        .map_err(CodegenError::BuildReturn)?;
    module
        .verify()
        .map_err(|message| CodegenError::Verification(message.to_string()))?;

    Ok(module)
}
