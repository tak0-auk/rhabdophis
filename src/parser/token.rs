use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub range: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Int(i128),
    Float(f64),
    String(String),
    Symbol(Symbol),
    Newline,
    Indent(String),
}

impl Token {

    pub fn new(kind: TokenKind, start: usize, end: usize) -> Token {
        Token {
            kind: kind,
            range: Range {
                start: start,
                end: end,
            }
        }
    }

    pub fn new_identifier(name: String, start: usize, end: usize) -> Token {
        Token {
            kind: TokenKind::Identifier(name),
            range: Range {
                start: start,
                end: end,
            }
        }
    }

    pub fn new_int(n: i128, start: usize, end: usize) -> Token {
        Token {
            kind: TokenKind::Int(n),
            range: Range {
                start: start,
                end: end,
            }
        }
    }

    pub fn new_float(f: f64, start: usize, end: usize) -> Token {
        Token {
            kind: TokenKind::Float(f),
            range: Range {
                start: start,
                end: end,
            }
        }
    }

    pub fn new_string(s: String, start: usize, end: usize) -> Token {
        Token {
            kind: TokenKind::String(s),
            range: Range {
                start: start,
                end: end,
            }

        }
    }

    pub fn new_symbol(symbol: Symbol, start: usize, end: usize) -> Token {
        Token {
            kind: TokenKind::Symbol(symbol),
            range: Range {
                start: start,
                end: end,
            }
        }
    }

    pub fn new_newline(start: usize, end: usize) -> Token {
        Token {
            kind: TokenKind::Newline,
            range: Range {
                start: start,
                end: end,
            }
        }
    }

    pub fn new_indent(s: String, start: usize, end: usize) -> Token {
        Token {
            kind: TokenKind::Indent(s),
            range: Range {
                start: start,
                end: end,
            }
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