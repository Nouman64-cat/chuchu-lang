use chuchu::ast::{BinaryOperator, Expr, ExprKind, Stmt, StmtKind};
use chuchu::lexer::tokenize;
use chuchu::parser::Parser;
use chuchu::token::{Span, TokenKind};

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
                        span: Span { start: 9, end: 11 },
                    },
                },
                span: Span { start: 0, end: 11 },
            },
            Stmt {
                kind: StmtKind::Set {
                    name: "y".to_owned(),
                    value: Expr {
                        kind: ExprKind::Integer("20".to_owned()),
                        span: Span { start: 21, end: 23 },
                    },
                },
                span: Span { start: 12, end: 23 },
            },
            Stmt {
                kind: StmtKind::Show(Expr {
                    kind: ExprKind::Binary {
                        left: Box::new(Expr {
                            kind: ExprKind::Identifier("x".to_owned()),
                            span: Span { start: 30, end: 31 },
                        }),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expr {
                            kind: ExprKind::Identifier("y".to_owned()),
                            span: Span { start: 34, end: 35 },
                        }),
                    },
                    span: Span { start: 30, end: 35 },
                }),
                span: Span { start: 25, end: 35 },
            },
        ]
    );
    assert_eq!(
        parser.current().map(|token| &token.kind),
        Some(&TokenKind::Eof)
    );
}
