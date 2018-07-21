#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Int(i128),
    Float(f64),
    String(String),
    Symbol(String),
    Newline,
    Number,
    Indent(String),
    Literal(String),
    Pass,
}

impl Token {

    pub fn new(kind: TokenKind) -> Token {
        Token {
            kind: kind,
        }
    }

    pub fn new_identifier(ident: String) -> Token {
        Token {
            kind: TokenKind::Identifier(ident),
        }
    }

    pub fn new_int(i: i128) ->Token {
        Token {
            kind: TokenKind::Int(i),
        }
    }



    pub fn new_symbol(s: String) -> Token {
        Token {
            kind: TokenKind::Symbol(s),
        }
    }

    pub fn new_newline() -> Token {
        Token {
            kind: TokenKind::Newline,
        }
    }

    pub fn new_indent(s: String) -> Token {
        Token {
            kind: TokenKind::Indent(s),
        }
    }

    pub fn new_literal(l: String) -> Token {
        Token {
            kind: TokenKind::Literal(l),
        }
    }

    pub fn new_pass() -> Token {
        Token {
            kind: TokenKind::Pass,
        }
    }
}
