#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// Variable declaration keyword: `assign`.
    Set,
    To,
    Show,

    Identifier(String),
    Integer(String),
    Float(String),
    True,
    False,
    If,
    Else,
    While,
    Function,
    Reassign,
    Return,

    Plus,
    Minus,
    Star,
    Slash,
    Assign,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Arrow,

    LeftBrace,
    RightBrace,

    Newline,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
