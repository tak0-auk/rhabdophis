use parser::token::Token;

#[derive(Debug)]
pub struct Lexer {
    pub source: String,
    pos: usize,
}

impl Lexer {
    pub fn new(src: String) -> Lexer {
        Lexer {
            source: src,
            pos: 0,
        }
    }
}

impl Lexer {
    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        while let Ok(token) = self.read_token() {
            tokens.push(token);
        }
        return tokens;
    }
}

impl Lexer {
    fn is_eos(&self) -> bool {
        self.source.len() <= self.pos
    }

    fn next(&self) -> Result<char, ()> {
        self.source[self.pos..].chars().next().ok_or(())
    }

    fn skip_while<F>(&mut self, mut f: F) -> Result<String, ()>
    where
        F: FnMut(char) -> bool,
        {
            let mut v = vec![];
            while !self.is_eos() && f(self.next()?) {
                v.push(self.skip_char()? as u8);
            }
            Ok(String::from_utf8_lossy(v.as_slice()).to_owned().to_string())
        }

    fn skip_char(&mut self) -> Result<char, ()> {
        let mut iter = self.source[self.pos..].char_indices();
        let (_, cur_char) = iter.next().ok_or(())?;
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        Ok(cur_char)
    }

}

impl Lexer {
    pub fn read_token(&mut self) -> Result<Token, ()> {
        match self.next()? {
            'a'...'z' | 'A'...'Z' | '_' => self.read_alphabet(),
            '0'...'9' => self.read_number(),
            '\n' | '\r' => self.read_newline(),
            c if c.is_whitespace() => {
                self.read_indent()
            }
            _ => self.read_symbol(),
        }
    }

    fn read_alphabet(&mut self) -> Result<Token, ()> {
        let alphabet = self.skip_while(|c| c.is_alphanumeric() || c == '_')?;
        Ok(Token::new_identifier(alphabet))
    }

    fn read_number(&mut self) -> Result<Token, ()> {
        let number = self.skip_while(|c| match c {
            '0'...'9' => true,
            _ => false,
        })?;
        Ok(Token::new_number(number))
    }

    fn read_newline(&mut self) -> Result<Token, ()> {
        let newline = self.skip_char()?;
        Ok(Token::new_newline(newline.to_string()))
    }

    fn read_indent(&mut self) -> Result<Token, ()> {
        let space = self.skip_while(|c| c.is_whitespace())?;
        Ok(Token::new_indent(space))
    }

    fn read_symbol(&mut self) -> Result<Token, ()> {
        Ok(Token::new_symbol(self.skip_char()?.to_string()))
    }
}

/////////////////////////////////////////////////////////////////////
///
///
/////////////////////////////////////////////////////////////////////

#[test]
fn it_faild() {
    let mut lex = Lexer::new("a = 1 + 1".to_string());
    assert_eq!(lex.get_tokens().first().unwrap().kind, TokenKind::Newline);

}