use std::ops::Range;

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
    Symbol(Symbol),
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

    pub fn new_symbol(symbol: Symbol) -> Token {
        Token {
            kind: TokenKind::Symbol(symbol),
            value: String::new(),
        }
    }

    pub fn new_newline() -> Token {
        Token {
            kind: TokenKind::Newline,
            value: String::new(),
        }
    }

    pub fn new_indent(s: String, start: usize, end: usize) -> Token {
        Token {
            kind: TokenKind::Indent,
            value: s,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    OpeningParen,
    ClosingParen,
    OpeningBrace,
    ClosingBrace,
    OpeningBoxBracket,
    ClosingBoxBracket,
    Comma,
    Semicolon,
    Colon,
    Point,
    Arrow,
    Inc,
    Dec,
    Add,
    Sub,
    Asterisk,
    Div,
    Mod,
    Not,
    BitwiseNot,
    Shl,
    Shr,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    And,
    Or,
    Xor,
    LAnd,
    LOr,
    Question,
    Assign,
    AssignAdd,
    AssignSub,
    AssignMul,
    AssignDiv,
    AssignMod,
    AssignShl,
    AssignShr,
    AssignAnd,
    AssignOr,
    AssignXor,
    AssignLAnd,
    AssignLOr,
    Hash,
}