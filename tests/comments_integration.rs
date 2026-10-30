use chuchu::compiler::CompileError;
use chuchu::diagnostic::render;
use chuchu::lexer::{LexErrorKind, tokenize};
use chuchu::parser::{ParseError, Parser};
use chuchu::token::{Span, TokenKind};
use std::path::Path;

#[test]
fn line_comments_end_at_newlines_or_eof_and_allow_arbitrary_text() {
    for comment in [
        "//TODO: @ 😀 /* ignored",
        "//NOTE: text",
        "//CRITICAL: text",
        "//",
    ] {
        let source = format!("assign x as 10 {comment}\r\nshow x // trailing");
        let statements = Parser::new(tokenize(&source).unwrap())
            .parse_program()
            .unwrap();
        assert_eq!(statements.len(), 2);
        assert_eq!(
            &source[statements[1].span.start..statements[1].span.end],
            "show x"
        );
        assert_eq!(tokenize(comment).unwrap().len(), 1);
    }
}

#[test]
fn block_comments_preserve_original_newlines_and_byte_offsets() {
    let source = "show 1/* 😀\r\n@\n*/show 2";
    let tokens = tokenize(source).unwrap();
    let newlines: Vec<_> = tokens
        .iter()
        .filter(|t| t.kind == TokenKind::Newline)
        .collect();
    assert_eq!(newlines.len(), 2);
    for (token, offset) in newlines
        .iter()
        .zip(source.match_indices('\n').map(|(i, _)| i))
    {
        assert_eq!(
            token.span,
            Span {
                start: offset,
                end: offset + 1
            }
        );
    }
    let statements = Parser::new(tokens).parse_program().unwrap();
    assert_eq!(statements.len(), 2);
    assert_eq!(
        &source[statements[1].span.start..statements[1].span.end],
        "show 2"
    );
}

#[test]
fn inline_comments_are_whitespace_and_do_not_merge_tokens() {
    let source =
        "assign/* name */x/* separator */as/* value */10\nreassign x as x + /* amount */5\nshow x";
    assert_eq!(
        Parser::new(tokenize(source).unwrap())
            .parse_program()
            .unwrap()
            .len(),
        3
    );
    assert!(matches!(
        Parser::new(tokenize("show 1/**/2").unwrap()).parse_program(),
        Err(ParseError::ExpectedStatementSeparator { .. })
    ));
    assert!(
        Parser::new(tokenize("show 1 + /* split\nline */2").unwrap())
            .parse_program()
            .is_err()
    );
}

#[test]
fn comments_do_not_change_division_or_string_contents() {
    let tokens = tokenize("show 12 / /**/ 3 // tail\nshow \"//TODO: /* literal */\"").unwrap();
    assert_eq!(
        tokens.iter().filter(|t| t.kind == TokenKind::Slash).count(),
        1
    );
    assert!(
        tokens
            .iter()
            .any(|t| t.kind == TokenKind::String("//TODO: /* literal */".into()))
    );
    assert_eq!(Parser::new(tokens).parse_program().unwrap().len(), 2);
}

#[test]
fn block_comments_close_at_the_first_marker_without_nesting() {
    for source in [
        "/**/",
        "/* // line marker */",
        "/* \" unmatched quote */",
        "/* outer /* inner */",
    ] {
        assert_eq!(tokenize(source).unwrap().len(), 1, "{source}");
    }
    assert_eq!(
        Parser::new(tokenize("/**//**/show 1/* tail */").unwrap())
            .parse_program()
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn unterminated_block_comments_report_the_opening_and_repair() {
    for source in ["/*", "/*/", "show 1\n  /*NOTE: 😀\r\nmissing end"] {
        let error = tokenize(source).unwrap_err();
        assert_eq!(error.kind, LexErrorKind::UnterminatedComment);
        assert_eq!(
            error.span,
            Span {
                start: source.find("/*").unwrap(),
                end: source.len()
            }
        );
        let error = CompileError::Lex(error);
        assert!(
            error
                .to_string()
                .starts_with("Unterminated block comment at ")
        );
        let diagnostic = render(&error, Path::new("comments.chuchu"), source);
        assert!(diagnostic.contains("Unterminated block comment"));
        assert!(diagnostic.contains("Close the block comment with */."));
    }
}
