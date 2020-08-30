#[derive(Debug, Copy, Clone)]
pub enum Token {
    Number(i64),
    BinOp(BinOp),
    LeftPar,
    RightPar,
    EOF,
}

#[derive(Debug, Copy, Clone)]
pub enum BinOp {
    Plus,  // +
    Minus, // -
    Mul,   // *
    Div,   // /
    Pow,   // **
}

pub type Tokens = Vec<Token>;
