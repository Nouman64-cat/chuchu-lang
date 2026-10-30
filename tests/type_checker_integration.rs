use chuchu::lexer::tokenize;
use chuchu::parser::Parser;
use chuchu::symbol_table::SymbolTable;
use chuchu::token::Span;
use chuchu::type_checker::{TypeChecker, TypeError};
use chuchu::types::Type;

#[test]
fn type_checks_complete_chuchu_program() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/main.chuchu");
    let source = std::fs::read_to_string(path).expect("Example program should be readable");
    let tokens = tokenize(&source).expect("Example program should tokenize");
    let statements = Parser::new(tokens)
        .parse_program()
        .expect("Example program should parse");
    let mut checker = TypeChecker::new(SymbolTable::new());

    assert_eq!(checker.check_program(&statements), Ok(()));
}

fn check(source: &str) -> Result<(), TypeError> {
    let statements = Parser::new(tokenize(source).unwrap())
        .parse_program()
        .expect("Program should parse");
    TypeChecker::new(SymbolTable::new()).check_program(&statements)
}

#[test]
fn type_checks_assignment_with_as() {
    assert_eq!(
        check("assign x as 15\nassign y as 2.5\nshow(x + y)\nreassign x as 30"),
        Ok(())
    );
    assert_eq!(
        check("assign x as 1\nreassign x as true"),
        Err(TypeError::Mismatch {
            expected: Type::Int,
            actual: Type::Bool,
            span: Span { start: 28, end: 32 },
        })
    );
    assert!(matches!(
        check("assign x as 1\nassign x as 2"),
        Err(TypeError::DuplicateDeclaration { .. })
    ));
    assert!(matches!(
        check("show(missing)"),
        Err(TypeError::UndefinedVariable { .. })
    ));
}
