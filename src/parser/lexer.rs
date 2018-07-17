use parser::token::{ Token, TokenKind };

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
            if token.kind != TokenKind::Pass {
                tokens.push(token);
            }

        }
        return tokens;
    }
}

impl Lexer {
    fn is_eos(&self) -> bool {
        self.source.len() <= self.pos
    }

    fn c(&self) -> Result<char, ()> {
        self.source[self.pos..].chars().next().ok_or(())
    }

    fn next(&mut self) -> Result<char, ()> {
        let ret = self.source[self.pos..].chars().next().ok_or(());
        self.pos += 1;
        return ret;
    }

    fn skip_while<F>(&mut self, mut f: F) -> Result<String, ()>
    where
        F: FnMut(char) -> bool,
        {
            let mut v = vec![];
            while !self.is_eos() && f(self.c()?) {
                v.push(self.next()? as u8);
            }
            Ok(String::from_utf8_lossy(v.as_slice()).to_owned().to_string())
        }

    // fn skip_char(&mut self) -> Result<char, ()> {
    //     let mut iter = self.source[self.pos..].char_indices();
    //     let (_, cur_char) = iter.next().ok_or(())?;
    //     let (next_pos, _) = iter.next().unwrap_or((1, ' '));
    //     self.pos += next_pos;
    //     Ok(cur_char)
    // }

}

impl Lexer {
    pub fn read_token(&mut self) -> Result<Token, ()> {
        // let line = &self.source.lines();
        match self.c()? {
            '\'' | '"' => self.letter_quote(),
            '#' => self.read_comment(),
            'a'...'z' | 'A'...'Z' | '_' => self.read_identifier(),
            '0'...'9' => self.read_number(),
            '\n' | '\r' => self.read_newline(),
            c if c.is_whitespace() => {
                self.read_indent()
            }
            _ => self.read_symbol(),
        }
    }

    fn read_identifier(&mut self) -> Result<Token, ()> {
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
        let newline = self.next()?;
        Ok(Token::new_newline(newline.to_string()))
    }

    fn read_indent(&mut self) -> Result<Token, ()> {
        let space = self.skip_while(|c| c.is_whitespace())?;
        Ok(Token::new_indent(space))
    }

    fn read_comment(&mut self) -> Result<Token, ()> {
        self.skip_while(|c| c != '\n');
        Ok(Token::new_pass())
    }

    fn read_symbol(&mut self) -> Result<Token, ()> {
        Ok(Token::new_symbol(self.next()?.to_string()))
    }
}

impl Lexer {
    fn letter_quote(&mut self) -> Result<Token, ()> {
        let quote = self.next()?;
        let mut quote_siz: usize = 1; // 1 or 3

        let mut v = vec![];

        // v.push(self.next()? as u8);
            // Ok(String::from_utf8_lossy(v.as_slice()).to_owned().to_string())

        let mut q = self.next()?;
        if quote == q {
            q = self.next()?;
            if quote == q {
                quote_siz = 3;
            }
        }

        let mut c: char;
        let mut end_quote_size = 0;
        while end_quote_size != quote_siz {
            c = self.next()?;
            if c == quote {
                end_quote_size += 1;
                continue
            }
            v.push(c as u8);
        }
        Ok(Token::new_literal(String::from_utf8_lossy(v.as_slice()).to_owned().to_string()))
    }
}

#[test]
fn test_c() {
    let lex = Lexer::new("a = 1 + 1".to_string());
    assert_eq!(lex.c().unwrap(), 'a');
    assert_eq!(lex.c().unwrap(), 'a');
}

#[test]
fn test_next() {
    let mut lex = Lexer::new("a= 1 + 1".to_string());
    assert_eq!(lex.c().unwrap(), 'a');
    assert_eq!(lex.next().unwrap(), 'a');
    assert_eq!(lex.c().unwrap(), '=');
}