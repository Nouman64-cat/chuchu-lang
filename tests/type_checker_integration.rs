use chuchu::lexer::tokenize;
use chuchu::parser::Parser;
use chuchu::symbol_table::SymbolTable;
use chuchu::type_checker::TypeChecker;

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
