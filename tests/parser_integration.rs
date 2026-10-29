use chuchu::ast::{BinaryOperator, Expr, ExprKind, Stmt, StmtKind};
use chuchu::lexer::tokenize;
use chuchu::parser::Parser;
use chuchu::token::{Span, TokenKind};

#[test]
fn assign_replaces_set_declarations_without_changing_reassignment() {
    let statements = Parser::new(tokenize("assign set 5\nreassign set 6\nshow set").unwrap())
        .parse_program()
        .unwrap();
    assert!(matches!(&statements[0].kind, StmtKind::Set { name, .. } if name == "set"));
    assert_eq!(statements[0].span, Span { start: 0, end: 12 });
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
    // The expected spans below are LF byte offsets; Git may check this fixture out
    // with CRLF on Windows. CRLF spans are covered by the dedicated tests below.
    let source = std::fs::read_to_string(path)
        .expect("Example program should be readable")
        .replace("\r\n", "\n");
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
                kind: StmtKind::Assign {
                    name: "x".to_owned(),
                    value: Expr {
                        kind: ExprKind::Integer("15".to_owned()),
                        span: Span { start: 36, end: 38 }
                    },
                },
                span: Span { start: 25, end: 38 },
            },
            Stmt {
                kind: StmtKind::Show(Expr {
                    kind: ExprKind::Binary {
                        left: Box::new(Expr {
                            kind: ExprKind::Identifier("x".to_owned()),
                            span: Span { start: 45, end: 46 },
                        }),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expr {
                            kind: ExprKind::Identifier("y".to_owned()),
                            span: Span { start: 49, end: 50 },
                        }),
                    },
                    span: Span { start: 45, end: 50 },
                }),
                span: Span { start: 40, end: 50 },
            },
        ]
    );
    assert_eq!(
        parser.current().map(|token| &token.kind),
        Some(&TokenKind::Eof)
    );
}

fn parse_lines(lines: &[&str], separator: &str) -> (String, Vec<Stmt>) {
    let source = lines.join(separator);
    let statements = Parser::new(tokenize(&source).expect("Program should tokenize"))
        .parse_program()
        .expect("Program should parse");
    (source, statements)
}

#[test]
fn crlf_and_lf_sources_parse_to_the_same_structure() {
    let lines = ["assign x 10", "reassign x 15", "show x + 1"];
    let (_, lf) = parse_lines(&lines, "\n");
    let (_, crlf) = parse_lines(&lines, "\r\n");

    assert_eq!(lf.len(), 3);
    assert_eq!(crlf.len(), 3);
    for (lf, crlf) in lf.iter().zip(&crlf) {
        assert_eq!(
            std::mem::discriminant(&lf.kind),
            std::mem::discriminant(&crlf.kind)
        );
    }
    assert!(matches!(
        &crlf[0].kind,
        StmtKind::Set { name, value } if name == "x" && value.kind == ExprKind::Integer("10".to_owned())
    ));
    assert!(matches!(
        &crlf[1].kind,
        StmtKind::Assign { name, value } if name == "x" && value.kind == ExprKind::Integer("15".to_owned())
    ));
    assert!(matches!(
        &crlf[2].kind,
        StmtKind::Show(Expr {
            kind: ExprKind::Binary {
                operator: BinaryOperator::Add,
                ..
            },
            ..
        })
    ));
}

#[test]
fn crlf_spans_are_original_input_byte_offsets() {
    let lines = ["assign x 10", "reassign x 15", "show x + 1"];
    let (source, statements) = parse_lines(&lines, "\r\n");

    // Each CRLF is two bytes, so statements begin at 0, 13 and 28.
    let spans: Vec<Span> = statements.iter().map(|statement| statement.span).collect();
    assert_eq!(
        spans,
        vec![
            Span { start: 0, end: 11 },
            Span { start: 13, end: 26 },
            Span { start: 28, end: 38 },
        ]
    );
    for (span, line) in spans.iter().zip(lines) {
        assert_eq!(&source[span.start..span.end], line);
    }
    let StmtKind::Set { value, .. } = &statements[0].kind else {
        panic!("first statement should be an assignment");
    };
    assert_eq!(&source[value.span.start..value.span.end], "10");
}

#[test]
fn crlf_with_blank_lines_keeps_spans_exact() {
    let source = "assign x 1\r\n\r\nshow x\r\n";
    let statements = Parser::new(tokenize(source).unwrap())
        .parse_program()
        .unwrap();

    assert_eq!(statements[0].span, Span { start: 0, end: 10 });
    assert_eq!(statements[1].span, Span { start: 14, end: 20 });
    assert_eq!(&source[14..20], "show x");
}

#[test]
fn crlf_lexical_errors_report_original_byte_offsets() {
    let error = tokenize("assign x 1\r\nshow @").unwrap_err();

    assert_eq!(error.character, '@');
    assert_eq!(error.span, Span { start: 17, end: 18 });
}
