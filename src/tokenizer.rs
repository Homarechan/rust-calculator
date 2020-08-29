use crate::token;

type Tokens = Vec<token::Token>;
type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    OutOfBound,
    Unknown,
    Unexpected,
    Unreachable,
}

#[derive(Debug)]
pub struct Tokenizer {
    expression: String,
    cursor: usize,
    tokens: Tokens,
}

impl Tokenizer {
    pub fn new(expression: String) -> Tokenizer {
        Tokenizer {
            expression,
            cursor: 0,
            tokens: Tokens::new(),
        }
    }

    pub fn get_tokens(self) -> Tokens {
        self.tokens
    }

    pub fn tokenize(&mut self) -> Result<()> {
        while self.cursor < self.expression.len() {
            let now = self.get_now();
            dbg!(&now);

            match now {
                Ok(' ') => {
                    self.next_char();
                }

                Ok('(') => {
                    self.next_char();
                    self.tokens.push(token::Token::LeftPar);
                }

                Ok(')') => {
                    self.next_char();
                    self.tokens.push(token::Token::RightPar);
                }

                Ok('\0') => {
                    self.new_token(token::Token::EOF);
                    break;
                }

                Ok(c) => {
                    if c.is_digit(10) {
                        let num = self.get_num();
                        self.new_num_token(num)?;
                    } else if "+-*/^".contains(c) {
                        let binop = self.get_binop();
                        self.new_binop_token(binop)?;
                    }
                }

                Err(_) => {}
            }
        }

        Ok(())
    }
}

impl Tokenizer {
    fn new_token(&mut self, token: token::Token) {
        self.tokens.push(token);
    }

    fn new_num_token(&mut self, num: Result<i64>) -> Result<()> {
        self.new_token(token::Token::Number(num?));
        Ok(())
    }

    fn new_binop_token(&mut self, binop: Result<token::BinOp>) -> Result<()> {
        self.new_token(token::Token::BinOp(binop?));
        Ok(())
    }

    fn next_char(&mut self) {
        self.cursor += 1;
    }

    fn prev_char(&mut self) {
        self.cursor -= 1;
    }

    fn get_now_and_next(&mut self) -> Result<char> {
        self.next_char();
        match self.expression.chars().nth(self.cursor - 1) {
            Some(c) => Ok(c),
            None => Err(Error::OutOfBound),
        }
    }

    fn get_now(&mut self) -> Result<char> {
        match self.expression.chars().nth(self.cursor) {
            Some(c) => Ok(c),
            None => Err(Error::OutOfBound),
        }
    }

    fn get_num(&mut self) -> Result<i64> {
        let mut str_num = String::new();

        loop {
            match self.get_now() {
                Ok(c) => {
                    dbg!(c);
                    if "1234567890".contains(c) {
                        str_num.push(c);
                        self.next_char();
                    } else {
                        break;
                    }
                }
                Err(_) => return Err(Error::OutOfBound),
            }
        }

        dbg!(&str_num);

        return str_num.parse::<i64>().or(Err(Error::Unreachable));
    }

    fn get_binop(&mut self) -> Result<token::BinOp> {
        use token::BinOp::*;

        match self.get_now_and_next() {
            Ok('+') => Ok(Plus),
            Ok('-') => Ok(Minus),
            Ok('*') => match self.get_now() {
                Ok('*') => {
                    self.next_char();
                    Ok(Pow)
                }
                Ok(_) => Ok(Mul),
                Err(e) => Err(e),
            },
            Ok('/') => Ok(Div),
            Ok('^') => Ok(Pow),
            Ok(_) => Err(Error::Unreachable),
            Err(e) => Err(e),
        }
    }
}
