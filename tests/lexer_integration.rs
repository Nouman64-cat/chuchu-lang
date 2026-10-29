use chuchu::lexer::tokenize;
use chuchu::token::TokenKind;

#[test]
fn assign_is_reserved_and_set_is_an_identifier() {
    let tokens = tokenize("assign set 5\nshow assignable").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Set);
    assert_eq!(tokens[0].span, chuchu::token::Span { start: 0, end: 6 });
    assert_eq!(tokens[1].kind, TokenKind::Identifier("set".to_owned()));
    assert_eq!(tokens[2].kind, TokenKind::Integer("5".to_owned()));
    assert_eq!(tokens[2].span, chuchu::token::Span { start: 11, end: 12 });
    assert_eq!(
        tokens[5].kind,
        TokenKind::Identifier("assignable".to_owned())
    );
}

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
            TokenKind::Integer("10".to_owned()),
            TokenKind::Set,
            TokenKind::Identifier("y".to_owned()),
            TokenKind::Integer("20".to_owned()),
            TokenKind::Reassign,
            TokenKind::Identifier("x".to_owned()),
            TokenKind::Integer("15".to_owned()),
            TokenKind::Show,
            TokenKind::Identifier("x".to_owned()),
            TokenKind::Plus,
            TokenKind::Identifier("y".to_owned()),
            TokenKind::Eof,
        ]
    );
}

#[test]
fn recognizes_function_and_reassign_keywords_and_identifier_boundaries() {
    let tokens = tokenize("function function_name reassign reassignment").unwrap();
    let kinds: Vec<TokenKind> = tokens.into_iter().map(|token| token.kind).collect();

    assert_eq!(
        kinds,
        vec![
            TokenKind::Function,
            TokenKind::Identifier("function_name".to_owned()),
            TokenKind::Reassign,
            TokenKind::Identifier("reassignment".to_owned()),
            TokenKind::Eof,
        ]
    );
}
