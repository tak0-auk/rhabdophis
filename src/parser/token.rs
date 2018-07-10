#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    Int(i128),
    Float(f64),
    String(String),
    Symbol,
    Newline,
    Number,
    Indent,
}

impl Token {

    pub fn new(kind: TokenKind, v: String) -> Token {
        Token {
            kind: kind,
            value: v
        }
    }

    pub fn new_identifier(name: String) -> Token {
        Token {
            kind: TokenKind::Identifier,
            value: name,
        }
    }

    pub fn new_number(n: String) -> Token {
        Token {
            kind: TokenKind::Number,
            value: n,
        }
    }

    pub fn new_symbol(s: String) -> Token {
        Token {
            kind: TokenKind::Symbol,
            value: s,
        }
    }

    pub fn new_newline(s: String) -> Token {
        Token {
            kind: TokenKind::Newline,
            value: s,
        }
    }

    pub fn new_indent(s: String) -> Token {
        Token {
            kind: TokenKind::Indent,
            value: s,
        }
    }
}
