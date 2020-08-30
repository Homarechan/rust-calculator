use crate::token;
use crate::token::Tokens;
use crate::tree;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnpairBracket,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Tokens,
    cursor: usize,
    tree: tree::Tree,
}

impl Parser {
    pub fn new(tokens: Tokens) -> Self {
        Self {
            tokens,
            cursor: 0,
            tree: tree::Tree::new(tree::Node::Head, None, None),
        }
    }

    pub fn get_tree(&self) -> &tree::Tree {
        &self.to_owned().tree
    }

    pub fn parse(&mut self) -> Result<()> {
        let r = self.expr();
        if let Ok(tree) = r {
            self.tree = tree;
            Ok(())
        } else {
            Err(r.err().unwrap())
        }
    }
}

impl Parser {
    fn take_now(&self) -> Option<&token::Token> {
        self.tokens.get(self.cursor)
    }

    fn next_token(&mut self) {
        self.cursor += 1;
    }

    fn expr(&mut self) -> Result<tree::Tree> {
        use token::*;
        let mut lhs = self.mul()?;

        match self.take_now() {
            Some(&token) => match token {
                Token::BinOp(BinOp::Plus) | Token::BinOp(BinOp::Minus) => {
                    self.next_token();
                    lhs = tree::Tree::new(
                        match token {
                            Token::BinOp(BinOp::Plus) => tree::Node::Plus,
                            Token::BinOp(BinOp::Minus) => tree::Node::Minus,
                            _ => unreachable!(),
                        },
                        Some(Box::new(lhs)),
                        Some(Box::new(self.mul()?)),
                    );
                }
                _ => {}
            },
            None => {}
        }

        Ok(lhs)
    }

    fn mul(&mut self) -> Result<tree::Tree> {
        use token::*;
        let mut lhs = self.primary()?;

        match self.take_now() {
            Some(&token) => match token {
                Token::BinOp(BinOp::Mul) | Token::BinOp(BinOp::Div) => {
                    self.next_token();
                    lhs = tree::Tree::new(
                        match token {
                            Token::BinOp(BinOp::Mul) => tree::Node::Mul,
                            Token::BinOp(BinOp::Div) => tree::Node::Div,
                            Token::BinOp(BinOp::Pow) => tree::Node::Pow,
                            _ => unreachable!(),
                        },
                        Some(Box::new(lhs)),
                        Some(Box::new(self.primary()?)),
                    );
                }
                _ => {}
            },
            None => {}
        }

        Ok(lhs)
    }

    fn primary(&mut self) -> Result<tree::Tree> {
        let ret;

        match self.take_now() {
            Some(token::Token::LeftPar) => {
                self.next_token();
                let node = self.expr()?;
                if let Some(token::Token::RightPar) = self.take_now() {
                    ret = Ok(node);
                } else {
                    ret = Err(Error::UnpairBracket);
                }
            }

            Some(token::Token::Number(n)) => {
                ret = Ok(tree::Tree::new(tree::Node::Num(n.to_owned()), None, None));
            }

            Some(_) => unreachable!(),

            None => unreachable!(),
        }

        self.next_token();

        ret
    }
}

pub fn hello() {
    print!("");
}
