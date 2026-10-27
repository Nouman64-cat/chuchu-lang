use crate::ast::{BinaryOperator, Expr, ExprKind, Stmt, StmtKind};
use crate::token::Span;
use crate::token::Token;
use crate::token::TokenKind;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ExpectedExpression { span: Span },
    ExpectedStatement { span: Span },
    ExpectedIdentifier { span: Span },
    ExpectedTo { span: Span },
    UnexpectedEnd,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    pub fn advance(&mut self) {
        if self.position + 1 < self.tokens.len() {
            self.position += 1;
        }
    }

    pub fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        let token = self.current().ok_or(ParseError::UnexpectedEnd)?;

        let start = token.span.start;
        let (kind, end) = match &token.kind {
            TokenKind::Show => {
                self.advance();
                let expression = self.parse_expression()?;
                let end = expression.span.end;
                (StmtKind::Show(expression), end)
            }
            TokenKind::Set => {
                self.advance();
                let token = self.current().ok_or(ParseError::UnexpectedEnd)?;
                let TokenKind::Identifier(name) = &token.kind else {
                    return Err(ParseError::ExpectedIdentifier { span: token.span });
                };
                let name = name.clone();
                self.advance();

                let token = self.current().ok_or(ParseError::UnexpectedEnd)?;
                if token.kind != TokenKind::To {
                    return Err(ParseError::ExpectedTo { span: token.span });
                }
                self.advance();

                let value = self.parse_expression()?;
                let end = value.span.end;
                (StmtKind::Set { name, value }, end)
            }
            _ => return Err(ParseError::ExpectedStatement { span: token.span }),
        };

        Ok(Stmt {
            kind,
            span: Span { start, end },
        })
    }

    pub fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_primary()?;

        while matches!(
            self.current().map(|token| &token.kind),
            Some(TokenKind::Plus)
        ) {
            self.advance();
            let right = self.parse_primary()?;
            let span = Span {
                start: left.span.start,
                end: right.span.end,
            };

            left = Expr {
                kind: ExprKind::Binary {
                    left: Box::new(left),
                    operator: BinaryOperator::Add,
                    right: Box::new(right),
                },
                span,
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.current().ok_or(ParseError::UnexpectedEnd)?;

        let kind = match &token.kind {
            TokenKind::Integer(value) => ExprKind::Integer(value.clone()),
            TokenKind::Identifier(name) => ExprKind::Identifier(name.clone()),
            _ => {
                return Err(ParseError::ExpectedExpression { span: token.span });
            }
        };

        let expr = Expr {
            kind,
            span: token.span,
        };

        self.advance();

        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::token::TokenKind;

    #[test]
    fn parses_variable_declaration() {
        for (source, start) in [("set x to 10", 0), ("  set x to 10  ", 2)] {
            let tokens = tokenize(source).expect("Lexer should succeed");
            let mut parser = Parser::new(tokens);

            assert_eq!(
                parser.parse_statement(),
                Ok(Stmt {
                    kind: StmtKind::Set {
                        name: "x".to_owned(),
                        value: Expr {
                            kind: ExprKind::Integer("10".to_owned()),
                            span: Span {
                                start: start + 9,
                                end: start + 11,
                            },
                        },
                    },
                    span: Span {
                        start,
                        end: start + 11
                    },
                })
            );
            assert_eq!(
                parser.current().map(|token| &token.kind),
                Some(&TokenKind::Eof)
            );
        }
    }

    #[test]
    fn parses_variable_declaration_with_addition() {
        let tokens = tokenize("set result to x + 20").expect("Lexer should succeed");
        let mut parser = Parser::new(tokens);

        assert_eq!(
            parser.parse_statement(),
            Ok(Stmt {
                kind: StmtKind::Set {
                    name: "result".to_owned(),
                    value: Expr {
                        kind: ExprKind::Binary {
                            left: Box::new(Expr {
                                kind: ExprKind::Identifier("x".to_owned()),
                                span: Span { start: 14, end: 15 },
                            }),
                            operator: BinaryOperator::Add,
                            right: Box::new(Expr {
                                kind: ExprKind::Integer("20".to_owned()),
                                span: Span { start: 18, end: 20 },
                            }),
                        },
                        span: Span { start: 14, end: 20 },
                    },
                },
                span: Span { start: 0, end: 20 },
            })
        );
        assert_eq!(
            parser.current().map(|token| &token.kind),
            Some(&TokenKind::Eof)
        );
    }

    #[test]
    fn rejects_variable_declaration_without_identifier() {
        for (source, span, kind) in [
            ("set to 10", Span { start: 4, end: 6 }, TokenKind::To),
            (
                "set 10",
                Span { start: 4, end: 6 },
                TokenKind::Integer("10".to_owned()),
            ),
            ("set", Span { start: 3, end: 3 }, TokenKind::Eof),
        ] {
            let tokens = tokenize(source).expect("Lexer should succeed");
            let mut parser = Parser::new(tokens);

            assert_eq!(
                parser.parse_statement(),
                Err(ParseError::ExpectedIdentifier { span })
            );
            assert_eq!(parser.current().map(|token| &token.kind), Some(&kind));
        }
    }

    #[test]
    fn rejects_variable_declaration_without_to() {
        for (source, span, kind) in [
            (
                "set x 10",
                Span { start: 6, end: 8 },
                TokenKind::Integer("10".to_owned()),
            ),
            ("set x show 10", Span { start: 6, end: 10 }, TokenKind::Show),
            ("set x", Span { start: 5, end: 5 }, TokenKind::Eof),
        ] {
            let tokens = tokenize(source).expect("Lexer should succeed");
            let mut parser = Parser::new(tokens);

            assert_eq!(
                parser.parse_statement(),
                Err(ParseError::ExpectedTo { span })
            );
            assert_eq!(parser.current().map(|token| &token.kind), Some(&kind));
        }
    }

    #[test]
    fn rejects_variable_declaration_without_valid_initializer() {
        for (source, span, kind) in [
            ("set x to", Span { start: 8, end: 8 }, TokenKind::Eof),
            ("set x to +", Span { start: 9, end: 10 }, TokenKind::Plus),
            ("set x to show", Span { start: 9, end: 13 }, TokenKind::Show),
            ("set x to 10 +", Span { start: 13, end: 13 }, TokenKind::Eof),
        ] {
            let tokens = tokenize(source).expect("Lexer should succeed");
            let mut parser = Parser::new(tokens);

            assert_eq!(
                parser.parse_statement(),
                Err(ParseError::ExpectedExpression { span })
            );
            assert_eq!(parser.current().map(|token| &token.kind), Some(&kind));
        }
    }

    #[test]
    fn parses_show_statement() {
        for (source, expression, span) in [
            (
                "show 42",
                Expr {
                    kind: ExprKind::Integer("42".to_owned()),
                    span: Span { start: 5, end: 7 },
                },
                Span { start: 0, end: 7 },
            ),
            (
                "  show x + 20",
                Expr {
                    kind: ExprKind::Binary {
                        left: Box::new(Expr {
                            kind: ExprKind::Identifier("x".to_owned()),
                            span: Span { start: 7, end: 8 },
                        }),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expr {
                            kind: ExprKind::Integer("20".to_owned()),
                            span: Span { start: 11, end: 13 },
                        }),
                    },
                    span: Span { start: 7, end: 13 },
                },
                Span { start: 2, end: 13 },
            ),
        ] {
            let tokens = tokenize(source).expect("Lexer should succeed");
            let mut parser = Parser::new(tokens);

            let statement = parser
                .parse_statement()
                .expect("Show statement should parse successfully");

            assert_eq!(
                statement,
                Stmt {
                    kind: StmtKind::Show(expression),
                    span,
                }
            );
            assert_eq!(
                parser.current().map(|token| &token.kind),
                Some(&TokenKind::Eof)
            );
        }

        for (source, expected) in [
            (
                "42",
                ParseError::ExpectedStatement {
                    span: Span { start: 0, end: 2 },
                },
            ),
            (
                "show",
                ParseError::ExpectedExpression {
                    span: Span { start: 4, end: 4 },
                },
            ),
            (
                "show +",
                ParseError::ExpectedExpression {
                    span: Span { start: 5, end: 6 },
                },
            ),
        ] {
            let tokens = tokenize(source).expect("Lexer should succeed");
            let mut parser = Parser::new(tokens);
            assert_eq!(parser.parse_statement(), Err(expected));
        }

        assert_eq!(
            Parser::new(Vec::new()).parse_statement(),
            Err(ParseError::UnexpectedEnd)
        );
    }

    #[test]
    fn parses_addition_expression() {
        let tokens = tokenize("10 + 20").expect("Lexer should succeed");
        let mut parser = Parser::new(tokens);

        let expression = parser
            .parse_expression()
            .expect("Addition should parse successfully");

        let expected = Expr {
            kind: ExprKind::Binary {
                left: Box::new(Expr {
                    kind: ExprKind::Integer("10".to_owned()),
                    span: Span { start: 0, end: 2 },
                }),
                operator: BinaryOperator::Add,
                right: Box::new(Expr {
                    kind: ExprKind::Integer("20".to_owned()),
                    span: Span { start: 5, end: 7 },
                }),
            },
            span: Span { start: 0, end: 7 },
        };

        assert_eq!(expression, expected);
        assert_eq!(
            parser.current().map(|token| &token.kind),
            Some(&TokenKind::Eof)
        );

        // Chained additions group to the left and can include identifiers.
        for (source, kind, end) in [
            ("10 + 20 + 30", ExprKind::Integer("30".to_owned()), 12),
            (
                "10 + 20 + total",
                ExprKind::Identifier("total".to_owned()),
                15,
            ),
        ] {
            let tokens = tokenize(source).expect("Lexer should succeed");
            let mut parser = Parser::new(tokens);

            let expression = parser
                .parse_expression()
                .expect("Chained addition should parse successfully");

            assert_eq!(
                expression,
                Expr {
                    kind: ExprKind::Binary {
                        left: Box::new(expected.clone()),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expr {
                            kind,
                            span: Span { start: 10, end },
                        }),
                    },
                    span: Span { start: 0, end },
                }
            );
            assert_eq!(
                parser.current().map(|token| &token.kind),
                Some(&TokenKind::Eof)
            );
        }
    }

    #[test]
    fn parses_identifier_expression() {
        let tokens = tokenize("total").expect("Lexer should succeed");
        let mut parser = Parser::new(tokens);

        let expression = parser
            .parse_expression()
            .expect("Identifier should parse successfully");

        assert_eq!(
            expression,
            Expr {
                kind: ExprKind::Identifier("total".to_owned()),
                span: Span { start: 0, end: 5 },
            }
        );

        assert_eq!(
            parser.current().map(|token| &token.kind),
            Some(&TokenKind::Eof)
        );
    }

    #[test]
    fn parses_integer_expression() {
        let tokens = tokenize("42").expect("Lexer should succeed");
        let mut parser = Parser::new(tokens);

        let expression = parser
            .parse_expression()
            .expect("Integer should parse successfully");

        assert_eq!(
            expression,
            Expr {
                kind: ExprKind::Integer("42".to_owned()),
                span: Span { start: 0, end: 2 },
            }
        );

        assert_eq!(
            parser.current().map(|token| &token.kind),
            Some(&TokenKind::Eof)
        );
    }

    #[test]
    fn advances_through_tokens() {
        let tokens = tokenize("{+}").expect("Lexer should succeed");
        let mut parser = Parser::new(tokens);

        parser.advance();

        assert_eq!(
            parser.current().map(|token| &token.kind),
            Some(&TokenKind::Plus)
        );

        parser.advance();
        parser.advance();

        assert_eq!(
            parser.current().map(|token| &token.kind),
            Some(&TokenKind::Eof)
        );

        // Advancing again should remain at EOF.
        parser.advance();

        assert_eq!(
            parser.current().map(|token| &token.kind),
            Some(&TokenKind::Eof)
        );
    }

    #[test]
    fn starts_at_first_token() {
        let tokens = tokenize("{+}").expect("Lexer should succeed");

        let parser = Parser::new(tokens);

        assert_eq!(
            parser.current().map(|token| &token.kind),
            Some(&TokenKind::LeftBrace)
        );
    }
}
