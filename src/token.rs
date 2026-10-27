#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Set,
    To,
    Show,

    Identifier(String),
    Integer(String),

    Plus,

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
