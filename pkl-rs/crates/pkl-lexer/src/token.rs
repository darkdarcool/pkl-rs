#[derive(Debug, Default, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Plus,
    PlusEq,
    Identifier,

    #[default]
    Empty,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new() -> Self {
        Token {
            kind: TokenKind::Empty,
            span: Span { start: 0, end: 0 },
        }
    }
}