use chuchu::lexer::tokenize;
use chuchu::token::TokenKind;

#[test]
fn tokenizes_complete_chuchu_program() {
    let source = include_str!("../examples/main.chuchu");

    let tokens = tokenize(source).expect("Valid Chuchu program should tokenize");

    let kinds: Vec<TokenKind> = tokens
        .into_iter()
        .map(|token| token.kind)
        .filter(|kind| !matches!(kind, TokenKind::Newline))
        .collect();

    assert_eq!(
        kinds,
        vec![
            TokenKind::Set,
            TokenKind::Identifier("x".to_owned()),
            TokenKind::To,
            TokenKind::Integer("10".to_owned()),
            TokenKind::Set,
            TokenKind::Identifier("y".to_owned()),
            TokenKind::To,
            TokenKind::Integer("20".to_owned()),
            TokenKind::Show,
            TokenKind::Identifier("x".to_owned()),
            TokenKind::Plus,
            TokenKind::Identifier("y".to_owned()),
            TokenKind::Eof,
        ]
    );
}
