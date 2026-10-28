use chuchu::ast::{BinaryOperator, Expr, ExprKind, Stmt, StmtKind};
use chuchu::lexer::tokenize;
use chuchu::parser::Parser;
use chuchu::token::{Span, TokenKind};

#[test]
fn assign_replaces_set_declarations_without_changing_reassignment() {
    let statements = Parser::new(tokenize("assign set to 5\nreassign set to 6\nshow set").unwrap())
        .parse_program()
        .unwrap();
    assert!(matches!(&statements[0].kind, StmtKind::Set { name, .. } if name == "set"));
    assert_eq!(statements[0].span, Span { start: 0, end: 15 });
    assert!(matches!(&statements[1].kind, StmtKind::Assign { name, .. } if name == "set"));
    assert_eq!(
        Parser::new(tokenize("set x to 5").unwrap()).parse_program(),
        Err(chuchu::parser::ParseError::ExpectedStatement {
            span: Span { start: 0, end: 3 }
        })
    );
}

#[test]
fn parses_complete_chuchu_program() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/main.chuchu");
    let source = std::fs::read_to_string(path).expect("Example program should be readable");
    let tokens = tokenize(&source).expect("Example program should tokenize");
    let mut parser = Parser::new(tokens);

    let statements = parser
        .parse_program()
        .expect("Example program should parse");

    assert_eq!(
        statements,
        vec![
            Stmt {
                kind: StmtKind::Set {
                    name: "x".to_owned(),
                    value: Expr {
                        kind: ExprKind::Integer("10".to_owned()),
                        span: Span { start: 12, end: 14 },
                    },
                },
                span: Span { start: 0, end: 14 },
            },
            Stmt {
                kind: StmtKind::Set {
                    name: "y".to_owned(),
                    value: Expr {
                        kind: ExprKind::Integer("20".to_owned()),
                        span: Span { start: 27, end: 29 },
                    },
                },
                span: Span { start: 15, end: 29 },
            },
            Stmt {
                kind: StmtKind::Assign {
                    name: "x".to_owned(),
                    value: Expr {
                        kind: ExprKind::Integer("15".to_owned()),
                        span: Span { start: 45, end: 47 }
                    },
                },
                span: Span { start: 31, end: 47 },
            },
            Stmt {
                kind: StmtKind::Show(Expr {
                    kind: ExprKind::Binary {
                        left: Box::new(Expr {
                            kind: ExprKind::Identifier("x".to_owned()),
                            span: Span { start: 54, end: 55 },
                        }),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expr {
                            kind: ExprKind::Identifier("y".to_owned()),
                            span: Span { start: 58, end: 59 },
                        }),
                    },
                    span: Span { start: 54, end: 59 },
                }),
                span: Span { start: 49, end: 59 },
            },
        ]
    );
    assert_eq!(
        parser.current().map(|token| &token.kind),
        Some(&TokenKind::Eof)
    );
}
