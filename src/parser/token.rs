#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Int(i128),
    Float(f64),
    Symbol(String),
    Number,
    Literal(String),
    EndMarker,
    NAME,
    NUMBER,
    String(String),
    NewLine,
    Indent(usize),
    Dedent,
    LPar,
    RPar,
    LSqb,
    RSqb,
    Colon,
    Comma,
    Semi,
    Plus,
    Minus,
    Star,
    Slash,
    VBar,
    Amper,
    Less,
    Greater,
    Equal,
    Dot,
    Percent,
    LBrace,
    RBrace,
    EqEqual,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Tilde,
    Circumflex,
    Leftshift,
    Rightshift,
    Doublestar,
    PlusEqual,
    MinEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    AmperEqual,
    VbarEqual,
    CircumflexEqual,
    LeftshiftEqual,
    RightshiftEqual,
    DoublestarEqual,
    Doubleslash,
    DoubleslashEqual,
    At,
    AtEqual,
    Rarrow,
    Ellipsis,
    /* This Table Must Match The #defines In Token.h! */
    Op,
    // <errortoken>,
    Comment,
    Nl,
    Encoding,
    // <n_tokens>,
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
            kind: TokenKind::NewLine,
        }
    }

    pub fn new_indent(s: usize) -> Token {
        Token {
            kind: TokenKind::Indent(s),
        }
    }

    pub fn new_string(s: String) -> Token {
        Token {
            kind: TokenKind::String(s),
        }
    }

}
