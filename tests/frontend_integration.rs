use chuchu::ast::{BinaryOperator as Op, ExprKind, Stmt, StmtKind};
use chuchu::lexer::tokenize;
use chuchu::parser::{ParseError, Parser};
use chuchu::symbol_table::SymbolTable;
use chuchu::token::{Span, TokenKind};
use chuchu::type_checker::{TypeChecker, TypeError};
use chuchu::types::{NumericType, Type};

fn parse(source: &str) -> Vec<Stmt> {
    Parser::new(tokenize(source).unwrap())
        .parse_program()
        .unwrap()
}
fn check(source: &str) -> Result<(), TypeError> {
    TypeChecker::new(SymbolTable::new()).check_program(&parse(source))
}

#[test]
fn lexes_tokens_with_utf8_error_spans() {
    let source = "- * / ( ) [ ] , : -> = == != < <= > >= true false if else while function reassign return 1.25 2e-3";
    let tokens = tokenize(source).unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Minus);
    assert!(
        tokens
            .iter()
            .any(|token| token.kind == TokenKind::Float("2e-3".to_owned()))
    );
    for token in tokens.iter().filter(|token| token.kind != TokenKind::Eof) {
        assert!(!source[token.span.start..token.span.end].is_empty());
    }
    assert_eq!(
        tokenize("show π").unwrap_err().span,
        Span { start: 5, end: 7 }
    );
}

#[test]
fn parses_precedence_associativity_and_grouping() {
    let expression = Parser::new(tokenize("10 - 2 * (3 + 1) - 1").unwrap())
        .parse_expression()
        .unwrap();
    assert_eq!(expression.span, Span { start: 0, end: 20 });
    let ExprKind::Binary {
        operator: Op::Subtract,
        left,
        right,
    } = expression.kind
    else {
        panic!("Expected subtraction");
    };
    assert_eq!(right.kind, ExprKind::Integer("1".to_owned()));
    let ExprKind::Binary {
        operator: Op::Subtract,
        right,
        ..
    } = left.kind
    else {
        panic!("Expected left associativity");
    };
    let ExprKind::Binary {
        operator: Op::Multiply,
        right,
        ..
    } = right.kind
    else {
        panic!("Expected multiplication precedence");
    };
    assert!(matches!(
        right.kind,
        ExprKind::Binary {
            operator: Op::Add,
            ..
        }
    ));
    assert_eq!(right.span, Span { start: 9, end: 16 });
}

#[test]
fn parses_functions_blocks_and_calls() {
    let statements =
        parse("function f(x: int, y: float) -> bool {\nreturn x < y\n}\nshow f(1, 2.0)");
    let StmtKind::Function {
        parameters,
        return_type,
        body,
        ..
    } = &statements[0].kind
    else {
        panic!("Expected function");
    };
    assert_eq!(parameters.len(), 2);
    assert_eq!(*return_type, Type::Bool);
    assert!(matches!(body[0].kind, StmtKind::Return(_)));
    assert!(
        matches!(&statements[1].kind, StmtKind::Show(expr) if matches!(expr.kind, ExprKind::Call { .. }))
    );
}

#[test]
fn rejects_missing_delimiters_and_separators() {
    for source in [
        "if true {\nshow 1",
        "if true show 1",
        "show (1 + 2",
        "show [1, 2",
        "show 1 show 2",
        "function f(x int) -> int { return x }",
    ] {
        let result = Parser::new(tokenize(source).unwrap()).parse_program();
        assert!(result.is_err(), "{source}");
    }
    assert!(matches!(
        Parser::new(tokenize("if true { show 1").unwrap()).parse_program(),
        Err(ParseError::ExpectedToken { .. })
    ));
}

#[test]
fn infers_scalar_and_array_types() {
    let checker = TypeChecker::new(SymbolTable::new());
    for (source, expected) in [
        ("1 + 2.5", Type::Float),
        ("true == false", Type::Bool),
        ("1 < 2.0", Type::Bool),
        (
            "[1, 2]",
            Type::Array {
                element: NumericType::Int,
                len: 2,
            },
        ),
        ("[1.0, 2.0][0]", Type::Float),
        ("sum([1, 2])", Type::Int),
    ] {
        let expression = Parser::new(tokenize(source).unwrap())
            .parse_expression()
            .unwrap();
        assert_eq!(
            checker.infer_expression(&expression),
            Ok(expected),
            "{source}"
        );
    }
}

#[test]
fn validates_scopes_shadowing_and_assignments() {
    assert_eq!(
        check("assign x 1\nif true {\nassign x 2\nreassign x 3\n}\nreassign x 4\nshow x"),
        Ok(())
    );
    assert!(matches!(
        check("if true { assign inside 1 }\nshow inside"),
        Err(TypeError::UndefinedVariable { .. })
    ));
    assert!(matches!(
        check("reassign missing 1"),
        Err(TypeError::UndefinedVariable { .. })
    ));
    assert!(matches!(
        check("assign x 1\nreassign x 2.0"),
        Err(TypeError::Mismatch { .. })
    ));
    assert_eq!(check("assign x 1.0\nreassign x 2"), Ok(()));
}

#[test]
fn validates_forward_recursive_and_typed_function_calls() {
    assert_eq!(
        check(
            "show f(3)\nfunction f(x: int) -> int {\nif x == 0 { return 1 } else { return f(x - 1) }\n}"
        ),
        Ok(())
    );
    for source in [
        "function f(x: int) -> int { show x }",
        "function f(x: int) -> int { return true }",
        "function f(x: int) -> int { return x }\nshow f(true)",
        "show missing(1)",
        "function f(x: int) -> int { return x }\nshow f(1, 2)",
        "function f(x: int, x: int) -> int { return x }",
        "return 1",
        "function f() -> int { return 1 }\nfunction f() -> int { return 2 }",
        "assign x 1\nfunction f() -> int { return x }",
        "function sqrt(x: float) -> float { return x }",
    ] {
        assert!(check(source).is_err(), "{source}");
    }
}

#[test]
fn rejects_invalid_operands_conditions_and_arrays() {
    for source in [
        "show true + 1",
        "if 1 { show 1 }",
        "while 1.0 { show 1 }",
        "show -true",
        "assign a []",
        "assign a [1, 2.0]",
        "assign a [true, false]",
        "assign a [[1], [2]]",
        "show [1][0.0]",
        "show 1[0]",
        "show [1]",
        "assign a [1, 2] + [1]",
        "assign a [1] * [1.0]",
        "show sqrt(1)",
    ] {
        assert!(check(source).is_err(), "{source}");
    }
    let oversized = format!("assign a [{}]", vec!["1"; 257].join(","));
    assert!(matches!(
        check(&oversized),
        Err(TypeError::InvalidArray { .. })
    ));
}

#[test]
fn rejects_invalid_float_literals_with_spans() {
    for source in ["show 1e", "show 1e9999"] {
        assert!(matches!(
            check(source),
            Err(TypeError::InvalidLiteral {
                span: Span { start: 5, .. },
                ..
            })
        ));
    }
}

#[test]
fn symbol_scopes_preserve_outer_bindings() {
    let mut symbols = SymbolTable::new();
    symbols.declare("x".to_owned(), Type::Int).unwrap();
    symbols.enter_scope();
    symbols.declare("x".to_owned(), Type::Float).unwrap();
    assert_eq!(symbols.lookup("x"), Some(Type::Float));
    assert!(symbols.declare("x".to_owned(), Type::Bool).is_err());
    symbols.exit_scope();
    assert_eq!(symbols.lookup("x"), Some(Type::Int));
}
