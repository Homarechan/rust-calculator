#[derive(Debug)]
pub enum Token {
    Number(i64),
    BinOp(BinOp),
    LeftPar,
    RightPar,
    EOF,
}

#[derive(Debug)]
pub enum BinOp {
    Plus,  // +
    Minus, // -
    Mul,   // *
    Div,   // /
    Pow,   // **
}
