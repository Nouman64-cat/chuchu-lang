use crate::ast::{BinaryOperator, Expr, ExprKind, Stmt, StmtKind};
use crate::symbol_table::{SymbolError, SymbolTable};
use crate::token::Span;
use crate::types::Type;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeError {
    UndefinedVariable { name: String, span: Span },
    DuplicateDeclaration { name: String, span: Span },
}

pub struct TypeChecker {
    symbols: SymbolTable,
}

impl TypeChecker {
    pub fn new(symbols: SymbolTable) -> Self {
        Self { symbols }
    }

    /// Checks statements in order, stopping at the first error.
    /// Declarations from earlier successful statements remain registered on failure.
    pub fn check_program(&mut self, statements: &[Stmt]) -> Result<(), TypeError> {
        for statement in statements {
            self.check_statement(statement)?;
        }
        Ok(())
    }

    pub fn check_statement(&mut self, stmt: &Stmt) -> Result<(), TypeError> {
        match &stmt.kind {
            StmtKind::Set { name, value } => {
                let ty = self.infer_expression(value)?;
                self.symbols
                    .declare(name.clone(), ty)
                    .map_err(|error| match error {
                        SymbolError::DuplicateDeclaration { name } => {
                            TypeError::DuplicateDeclaration {
                                name,
                                span: stmt.span,
                            }
                        }
                    })
            }
            StmtKind::Show(expr) => {
                self.infer_expression(expr)?;
                Ok(())
            }
        }
    }

    pub fn infer_expression(&self, expr: &Expr) -> Result<Type, TypeError> {
        match &expr.kind {
            ExprKind::Integer(_) => Ok(Type::Int),
            ExprKind::Identifier(name) => {
                self.symbols
                    .lookup(name)
                    .ok_or_else(|| TypeError::UndefinedVariable {
                        name: name.clone(),
                        span: expr.span,
                    })
            }
            ExprKind::Binary {
                left,
                operator: BinaryOperator::Add,
                right,
            } => {
                let left_type = self.infer_expression(left)?;
                let right_type = self.infer_expression(right)?;

                match (left_type, right_type) {
                    (Type::Int, Type::Int) => Ok(Type::Int),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::Parser;

    fn parse_expression(source: &str) -> Expr {
        let tokens = tokenize(source).expect("Expression should tokenize");
        Parser::new(tokens)
            .parse_expression()
            .expect("Expression should parse")
    }

    fn parse_statement(source: &str) -> Stmt {
        let tokens = tokenize(source).expect("Statement should tokenize");
        Parser::new(tokens)
            .parse_statement()
            .expect("Statement should parse")
    }

    fn parse_program(source: &str) -> Vec<Stmt> {
        let tokens = tokenize(source).expect("Program should tokenize");
        Parser::new(tokens)
            .parse_program()
            .expect("Program should parse")
    }

    #[test]
    fn checks_valid_program() {
        let statements = parse_program("set x to 10\nset y to x + 20\nshow y");
        let mut checker = TypeChecker::new(SymbolTable::new());

        assert_eq!(checker.check_program(&statements), Ok(()));
        assert_eq!(checker.symbols.lookup("x"), Some(Type::Int));
        assert_eq!(checker.symbols.lookup("y"), Some(Type::Int));
    }

    #[test]
    fn checks_empty_program() {
        let statements = parse_program("");
        let mut symbols = SymbolTable::new();
        symbols
            .declare("existing".to_owned(), Type::Int)
            .expect("Declaration should succeed");
        let mut checker = TypeChecker::new(symbols);

        assert_eq!(checker.check_program(&statements), Ok(()));
        assert_eq!(checker.symbols.lookup("existing"), Some(Type::Int));
        assert_eq!(
            TypeChecker::new(SymbolTable::new()).check_program(&[]),
            Ok(())
        );
    }

    #[test]
    fn checks_program_declaration_using_previous_variable() {
        let statements = parse_program("set x to 10\nset y to x\nset z to y + x");
        let mut checker = TypeChecker::new(SymbolTable::new());

        assert_eq!(checker.check_program(&statements), Ok(()));
        for name in ["x", "y", "z"] {
            assert_eq!(checker.symbols.lookup(name), Some(Type::Int));
        }
    }

    #[test]
    fn rejects_program_with_undefined_variable() {
        let statements = parse_program("set x to 10\nshow unknown");
        let mut checker = TypeChecker::new(SymbolTable::new());

        assert_eq!(
            checker.check_program(&statements),
            Err(TypeError::UndefinedVariable {
                name: "unknown".to_owned(),
                span: Span { start: 17, end: 24 },
            })
        );
    }

    #[test]
    fn rejects_program_with_duplicate_declaration() {
        let statements = parse_program("set x to 10\nset x to 20\nset later to 30");
        let mut checker = TypeChecker::new(SymbolTable::new());

        assert_eq!(
            checker.check_program(&statements),
            Err(TypeError::DuplicateDeclaration {
                name: "x".to_owned(),
                span: Span { start: 12, end: 23 },
            })
        );
        assert_eq!(checker.symbols.lookup("x"), Some(Type::Int));
        assert_eq!(checker.symbols.lookup("later"), None);
    }

    #[test]
    fn rejects_program_reference_before_declaration() {
        let statements = parse_program("show x\nset x to 10");
        let mut checker = TypeChecker::new(SymbolTable::new());

        assert_eq!(
            checker.check_program(&statements),
            Err(TypeError::UndefinedVariable {
                name: "x".to_owned(),
                span: Span { start: 5, end: 6 },
            })
        );
        assert_eq!(checker.symbols.lookup("x"), None);
    }

    #[test]
    fn program_stops_at_first_error_and_keeps_previous_declarations() {
        let statements =
            parse_program("set x to 10\nset y to unknown\nset later to 20\nshow other");
        let mut checker = TypeChecker::new(SymbolTable::new());

        assert_eq!(
            checker.check_program(&statements),
            Err(TypeError::UndefinedVariable {
                name: "unknown".to_owned(),
                span: Span { start: 21, end: 28 },
            })
        );
        assert_eq!(checker.symbols.lookup("x"), Some(Type::Int));
        assert_eq!(checker.symbols.lookup("y"), None);
        assert_eq!(checker.symbols.lookup("later"), None);
    }

    #[test]
    fn checks_declaration_and_subsequent_reference() {
        let mut checker = TypeChecker::new(SymbolTable::new());

        assert_eq!(
            checker.check_statement(&parse_statement("set x to 10")),
            Ok(())
        );
        assert_eq!(checker.symbols.lookup("x"), Some(Type::Int));
        assert_eq!(
            checker.infer_expression(&parse_expression("x")),
            Ok(Type::Int)
        );
    }

    #[test]
    fn checks_declaration_using_previously_declared_variable() {
        let mut checker = TypeChecker::new(SymbolTable::new());
        checker
            .check_statement(&parse_statement("set x to 10"))
            .expect("First declaration should succeed");

        assert_eq!(
            checker.check_statement(&parse_statement("set y to x + 20")),
            Ok(())
        );
        assert_eq!(checker.symbols.lookup("x"), Some(Type::Int));
        assert_eq!(checker.symbols.lookup("y"), Some(Type::Int));
    }

    #[test]
    fn checks_show_without_changing_symbols() {
        let mut checker = TypeChecker::new(SymbolTable::new());
        checker
            .check_statement(&parse_statement("set x to 10"))
            .expect("Declaration should succeed");

        for source in ["show 42", "show x", "show x + 20"] {
            assert_eq!(checker.check_statement(&parse_statement(source)), Ok(()));
            assert_eq!(checker.symbols.lookup("x"), Some(Type::Int));
        }
        // Output does not create a declaration or replace the existing one.
        assert_eq!(checker.symbols.lookup("unknown"), None);
        assert_eq!(
            checker.check_statement(&parse_statement("set x to 20")),
            Err(TypeError::DuplicateDeclaration {
                name: "x".to_owned(),
                span: Span { start: 0, end: 11 },
            })
        );
    }

    #[test]
    fn rejects_duplicate_declaration_with_statement_span() {
        let mut checker = TypeChecker::new(SymbolTable::new());
        checker
            .check_statement(&parse_statement("set x to 10"))
            .expect("First declaration should succeed");

        assert_eq!(
            checker.check_statement(&parse_statement("  set x to 20")),
            Err(TypeError::DuplicateDeclaration {
                name: "x".to_owned(),
                span: Span { start: 2, end: 13 },
            })
        );
        assert_eq!(checker.symbols.lookup("x"), Some(Type::Int));
    }

    #[test]
    fn rejects_show_with_undefined_variable() {
        let mut checker = TypeChecker::new(SymbolTable::new());

        assert_eq!(
            checker.check_statement(&parse_statement("show unknown + 10")),
            Err(TypeError::UndefinedVariable {
                name: "unknown".to_owned(),
                span: Span { start: 5, end: 12 },
            })
        );
        assert_eq!(checker.symbols.lookup("unknown"), None);
    }

    #[test]
    fn failed_initializer_does_not_register_variable() {
        let mut checker = TypeChecker::new(SymbolTable::new());

        for (source, name, undefined, span) in [
            (
                "set x to unknown + 10",
                "x",
                "unknown",
                Span { start: 9, end: 16 },
            ),
            ("set y to y + 10", "y", "y", Span { start: 9, end: 10 }),
        ] {
            assert_eq!(
                checker.check_statement(&parse_statement(source)),
                Err(TypeError::UndefinedVariable {
                    name: undefined.to_owned(),
                    span,
                })
            );
            assert_eq!(checker.symbols.lookup(name), None);
        }

        assert_eq!(
            checker.check_statement(&parse_statement("set x to 10")),
            Ok(())
        );
        assert_eq!(checker.symbols.lookup("x"), Some(Type::Int));
    }

    #[test]
    fn infers_integer_literal_type() {
        let checker = TypeChecker::new(SymbolTable::new());
        let expression = parse_expression("42");

        assert_eq!(checker.infer_expression(&expression), Ok(Type::Int));
    }

    #[test]
    fn infers_declared_identifier_type() {
        let mut symbols = SymbolTable::new();
        symbols
            .declare("total".to_owned(), Type::Int)
            .expect("Declaration should succeed");
        let checker = TypeChecker::new(symbols);

        assert_eq!(
            checker.infer_expression(&parse_expression("total")),
            Ok(Type::Int)
        );
    }

    #[test]
    fn reports_undefined_identifier_with_span() {
        let checker = TypeChecker::new(SymbolTable::new());

        assert_eq!(
            checker.infer_expression(&parse_expression("  unknown")),
            Err(TypeError::UndefinedVariable {
                name: "unknown".to_owned(),
                span: Span { start: 2, end: 9 },
            })
        );
    }

    #[test]
    fn infers_integer_addition_type() {
        let checker = TypeChecker::new(SymbolTable::new());

        for source in ["10 + 20", "10 + 20 + 30"] {
            assert_eq!(
                checker.infer_expression(&parse_expression(source)),
                Ok(Type::Int)
            );
        }
    }

    #[test]
    fn infers_addition_with_declared_identifier() {
        let mut symbols = SymbolTable::new();
        symbols
            .declare("x".to_owned(), Type::Int)
            .expect("Declaration should succeed");
        let checker = TypeChecker::new(symbols);

        for source in ["x + 20", "20 + x", "x + x + 20"] {
            assert_eq!(
                checker.infer_expression(&parse_expression(source)),
                Ok(Type::Int)
            );
        }
    }

    #[test]
    fn propagates_undefined_addition_operand_errors() {
        let checker = TypeChecker::new(SymbolTable::new());

        for (source, span) in [
            ("unknown + 10", Span { start: 0, end: 7 }),
            ("10 + unknown", Span { start: 5, end: 12 }),
            ("10 + 20 + unknown", Span { start: 10, end: 17 }),
            ("10 + unknown + 20", Span { start: 5, end: 12 }),
        ] {
            assert_eq!(
                checker.infer_expression(&parse_expression(source)),
                Err(TypeError::UndefinedVariable {
                    name: "unknown".to_owned(),
                    span,
                })
            );
        }
    }
}
