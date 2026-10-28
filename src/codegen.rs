use std::fmt;

use inkwell::builder::{Builder, BuilderError};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::IntValue;

use crate::ast::{BinaryOperator, Expr, ExprKind};
use crate::token::Span;

#[derive(Debug)]
pub enum CodegenError {
    BuildReturn(BuilderError),
    Verification(String),
    InvalidIntegerLiteral { literal: String, span: Span },
    UnsupportedExpression { span: Span },
    InvalidInsertionPoint { span: Span },
    BuildAddition { source: BuilderError, span: Span },
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
            Self::InvalidIntegerLiteral { literal, span } => write!(
                formatter,
                "Invalid or out-of-range signed 64-bit integer literal {literal:?} at {}..{}",
                span.start, span.end
            ),
            Self::UnsupportedExpression { span } => write!(
                formatter,
                "Unsupported expression for LLVM code generation at {}..{}",
                span.start, span.end
            ),
            Self::InvalidInsertionPoint { span } => write!(
                formatter,
                "Addition requires a builder positioned in an unterminated function block at {}..{}",
                span.start, span.end
            ),
            Self::BuildAddition { source, span } => write!(
                formatter,
                "Failed to build addition at {}..{}: {source}",
                span.start, span.end
            ),
        }
    }
}

impl std::error::Error for CodegenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BuildReturn(error) => Some(error),
            Self::BuildAddition { source, .. } => Some(source),
            Self::Verification(_)
            | Self::InvalidIntegerLiteral { .. }
            | Self::UnsupportedExpression { .. }
            | Self::InvalidInsertionPoint { .. } => None,
        }
    }
}

/// Generates integer literals and additions using a builder from the same context.
/// Additions require the builder to be positioned in an unterminated function block.
/// Signed i64 is a provisional backend representation, not a language-level width guarantee.
/// Signed overflow semantics remain undecided; additions set neither `nsw` nor `nuw`.
pub fn generate_expression<'ctx>(
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    expr: &Expr,
) -> Result<IntValue<'ctx>, CodegenError> {
    match &expr.kind {
        ExprKind::Integer(literal) => {
            let value =
                literal
                    .parse::<i64>()
                    .map_err(|_| CodegenError::InvalidIntegerLiteral {
                        literal: literal.clone(),
                        span: expr.span,
                    })?;
            Ok(context.i64_type().const_int(value.cast_unsigned(), true))
        }
        ExprKind::Binary {
            left,
            operator: BinaryOperator::Add,
            right,
        } => {
            let block = builder
                .get_insert_block()
                .ok_or(CodegenError::InvalidInsertionPoint { span: expr.span })?;
            if block.get_parent().is_none() || block.get_terminator().is_some() {
                return Err(CodegenError::InvalidInsertionPoint { span: expr.span });
            }
            let left = generate_expression(context, builder, left)?;
            let right = generate_expression(context, builder, right)?;
            builder.build_int_add(left, right, "add").map_err(|source| {
                CodegenError::BuildAddition {
                    source,
                    span: expr.span,
                }
            })
        }
        ExprKind::Identifier(_) => Err(CodegenError::UnsupportedExpression { span: expr.span }),
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
