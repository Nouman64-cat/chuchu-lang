use crate::token::{Span, Token, TokenKind};

#[derive(Debug, PartialEq, Eq)]
pub struct LexError {
    pub character: char,
    pub span: Span,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();

    let mut characters = source.char_indices().peekable();

    while let Some((start, character)) = characters.next() {
        let mut end = start + character.len_utf8();

        let kind = match character {
            'a'..='z' | 'A'..='Z' | '_' => {
                while let Some(&(position, next_char)) = characters.peek() {
                    if next_char.is_ascii_alphanumeric() || next_char == '_' {
                        characters.next();
                        end = position + next_char.len_utf8();
                    } else {
                        break;
                    }
                }

                match &source[start..end] {
                    "set" => TokenKind::Set,
                    "to" => TokenKind::To,
                    "show" => TokenKind::Show,
                    name => TokenKind::Identifier(name.to_owned()),
                }
            }
            '0'..='9' => {
                while let Some(&(position, next_char)) = characters.peek() {
                    if next_char.is_ascii_digit() {
                        characters.next();
                        end = position + next_char.len_utf8();
                    } else {
                        break;
                    }
                }

                TokenKind::Integer(source[start..end].to_owned())
            }
            '+' => TokenKind::Plus,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '\n' => TokenKind::Newline,
            ' ' | '\t' | '\r' => continue,
            _ => {
                return Err(LexError {
                    character,
                    span: Span { start, end },
                });
            }
        };

        tokens.push(Token {
            kind,
            span: Span { start, end },
        });
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        span: Span {
            start: source.len(),
            end: source.len(),
        },
    });

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_integer_literals() {
        let tokens = tokenize("10 2048").expect("Lexer should recognize integers");

        let kinds: Vec<TokenKind> = tokens.into_iter().map(|token| token.kind).collect();

        assert_eq!(
            kinds,
            vec![
                TokenKind::Integer("10".to_owned()),
                TokenKind::Integer("2048".to_owned()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn tokenizes_keywords_and_identifiers() {
        let tokens = tokenize("set total_1 to show").expect("Lexer should succeed");

        let kinds: Vec<TokenKind> = tokens.into_iter().map(|token| token.kind).collect();

        assert_eq!(
            kinds,
            vec![
                TokenKind::Set,
                TokenKind::Identifier("total_1".to_owned()),
                TokenKind::To,
                TokenKind::Show,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn tokenizes_single_character_symbols() {
        let tokens = tokenize("{+}").expect("Lexer should succeed");

        let kinds: Vec<TokenKind> = tokens.into_iter().map(|token| token.kind).collect();

        assert_eq!(
            kinds,
            vec![
                TokenKind::LeftBrace,
                TokenKind::Plus,
                TokenKind::RightBrace,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn rejects_invalid_character_with_location() {
        let error = tokenize("+@").expect_err("Lexer should reject invalid characters");

        assert_eq!(
            error,
            LexError {
                character: '@',
                span: Span { start: 1, end: 2 },
            }
        );
    }
}
