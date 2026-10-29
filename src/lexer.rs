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
                    "assign" => TokenKind::Set,
                    "show" => TokenKind::Show,
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    "if" => TokenKind::If,
                    "else" => TokenKind::Else,
                    "while" => TokenKind::While,
                    "function" => TokenKind::Function,
                    "reassign" => TokenKind::Reassign,
                    "return" => TokenKind::Return,
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

                let mut float = false;
                if characters.peek().is_some_and(|&(_, c)| c == '.') {
                    float = true;
                    if let Some((position, _)) = characters.next() {
                        end = position + 1;
                    }
                    while let Some(&(position, c)) = characters.peek() {
                        if !c.is_ascii_digit() {
                            break;
                        }
                        characters.next();
                        end = position + 1;
                    }
                }
                if characters
                    .peek()
                    .is_some_and(|&(_, c)| c == 'e' || c == 'E')
                {
                    float = true;
                    if let Some((position, _)) = characters.next() {
                        end = position + 1;
                    }
                    if characters
                        .peek()
                        .is_some_and(|&(_, c)| c == '+' || c == '-')
                        && let Some((position, _)) = characters.next()
                    {
                        end = position + 1;
                    }
                    while let Some(&(position, c)) = characters.peek() {
                        if !c.is_ascii_digit() {
                            break;
                        }
                        characters.next();
                        end = position + 1;
                    }
                }
                if float {
                    TokenKind::Float(source[start..end].to_owned())
                } else {
                    TokenKind::Integer(source[start..end].to_owned())
                }
            }
            '+' => TokenKind::Plus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            '-' | '=' | '!' | '<' | '>' => {
                let pair = characters
                    .peek()
                    .is_some_and(|&(_, c)| if character == '-' { c == '>' } else { c == '=' });
                if pair && let Some((position, _)) = characters.next() {
                    end = position + 1;
                }
                match (character, pair) {
                    ('-', true) => TokenKind::Arrow,
                    ('-', false) => TokenKind::Minus,
                    ('=', true) => TokenKind::Equal,
                    ('=', false) => TokenKind::Assign,
                    ('!', true) => TokenKind::NotEqual,
                    ('<', true) => TokenKind::LessEqual,
                    ('<', false) => TokenKind::Less,
                    ('>', true) => TokenKind::GreaterEqual,
                    ('>', false) => TokenKind::Greater,
                    _ => {
                        return Err(LexError {
                            character,
                            span: Span { start, end },
                        });
                    }
                }
            }
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
    fn operators_and_float_exponents_have_exact_spans() {
        let tokens = tokenize("-1.5e-2<=0").unwrap();
        assert_eq!(
            tokens[0],
            Token {
                kind: TokenKind::Minus,
                span: Span { start: 0, end: 1 }
            }
        );
        assert_eq!(
            tokens[1],
            Token {
                kind: TokenKind::Float("1.5e-2".to_owned()),
                span: Span { start: 1, end: 7 }
            }
        );
        assert_eq!(
            tokens[2],
            Token {
                kind: TokenKind::LessEqual,
                span: Span { start: 7, end: 9 }
            }
        );
    }

    #[test]
    fn keywords_require_identifier_boundaries() {
        let tokens = tokenize("while1 if_value return_value function_name reassignment").unwrap();
        assert!(
            tokens[..5]
                .iter()
                .all(|token| matches!(token.kind, TokenKind::Identifier(_)))
        );
    }

    #[test]
    fn recognizes_function_and_reassign_keywords() {
        let tokens = tokenize("function reassign function_name reassignment").unwrap();
        let kinds: Vec<TokenKind> = tokens.into_iter().map(|token| token.kind).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Function,
                TokenKind::Reassign,
                TokenKind::Identifier("function_name".to_owned()),
                TokenKind::Identifier("reassignment".to_owned()),
                TokenKind::Eof,
            ]
        );
    }

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
        let tokens = tokenize("assign total_1 show").expect("Lexer should succeed");

        let kinds: Vec<TokenKind> = tokens.into_iter().map(|token| token.kind).collect();

        assert_eq!(
            kinds,
            vec![
                TokenKind::Set,
                TokenKind::Identifier("total_1".to_owned()),
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
