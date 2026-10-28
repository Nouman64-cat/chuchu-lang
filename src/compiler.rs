use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use inkwell::context::Context;

use crate::codegen::{CodegenError, generate_program};
use crate::lexer::{LexError, tokenize};
use crate::native::{NativeError, build_executable};
use crate::parser::{ParseError, Parser};
use crate::symbol_table::SymbolTable;
use crate::type_checker::{TypeChecker, TypeError};

#[derive(Debug)]
pub enum CompileError {
    UnsupportedExtension(PathBuf),
    SourceRead { path: PathBuf, source: io::Error },
    InputOutputConflict(PathBuf),
    Lex(LexError),
    Parse(ParseError),
    Type(TypeError),
    Codegen(CodegenError),
    Native(NativeError),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedExtension(path) => {
                write!(f, "Expected a .chuchu source file: {}", path.display())
            }
            Self::SourceRead { path, source } => {
                write!(f, "Cannot read source {}: {source}", path.display())
            }
            Self::InputOutputConflict(path) => {
                write!(f, "Output would overwrite source {}", path.display())
            }
            Self::Lex(error) => write!(
                f,
                "Invalid source character {:?} at {}..{}",
                error.character, error.span.start, error.span.end
            ),
            Self::Parse(error) => {
                let (expected, span) = match error {
                    ParseError::UnexpectedEnd => {
                        return write!(f, "Unexpected end of token stream");
                    }
                    ParseError::ExpectedExpression { span } => ("expression", span),
                    ParseError::ExpectedStatement { span } => ("statement", span),
                    ParseError::ExpectedIdentifier { span } => ("identifier", span),
                    ParseError::ExpectedTo { span } => ("'to' keyword", span),
                    ParseError::ExpectedStatementSeparator { span } => {
                        ("newline between statements", span)
                    }
                };
                write!(f, "Expected {expected} at {}..{}", span.start, span.end)
            }
            Self::Type(error) => match error {
                TypeError::UndefinedVariable { name, span } => write!(
                    f,
                    "Undefined variable {name:?} at {}..{}",
                    span.start, span.end
                ),
                TypeError::DuplicateDeclaration { name, span } => write!(
                    f,
                    "Duplicate declaration of {name:?} at {}..{}",
                    span.start, span.end
                ),
            },
            Self::Codegen(error) => error.fmt(f),
            Self::Native(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CompileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SourceRead { source, .. } => Some(source),
            Self::Codegen(error) => Some(error),
            Self::Native(error) => Some(error),
            _ => None,
        }
    }
}

/// Validates the entire source before constructing LLVM IR or writing output files.
pub fn compile_source(source: &str, output: &Path) -> Result<(), CompileError> {
    let tokens = tokenize(source).map_err(CompileError::Lex)?;
    let statements = Parser::new(tokens)
        .parse_program()
        .map_err(CompileError::Parse)?;
    TypeChecker::new(SymbolTable::new())
        .check_program(&statements)
        .map_err(CompileError::Type)?;
    let context = Context::create();
    let module = generate_program(&context, &statements).map_err(CompileError::Codegen)?;
    build_executable(&module, output).map_err(CompileError::Native)
}

pub fn compile_file(input: &Path, output: &Path) -> Result<(), CompileError> {
    if input
        .extension()
        .is_none_or(|extension| extension != "chuchu")
    {
        return Err(CompileError::UnsupportedExtension(input.to_owned()));
    }
    let source = std::fs::read_to_string(input).map_err(|source| CompileError::SourceRead {
        path: input.to_owned(),
        source,
    })?;
    // Existing output aliases (including symlinks) must not destroy the input source.
    if let (Ok(input_path), Ok(output_path)) = (input.canonicalize(), output.canonicalize())
        && input_path == output_path
    {
        return Err(CompileError::InputOutputConflict(input.to_owned()));
    }
    compile_source(&source, output)
}
