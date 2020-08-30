#[derive(Debug, Copy, Clone)]
pub enum Node {
    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    Num(i64),
    Head, // leading temporary element
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub node: Node,

    // None if it is the end of a bipartite tree
    pub lhs: Option<Box<Tree>>,
    pub rhs: Option<Box<Tree>>,
}

impl Tree {
    pub fn new(node: Node, lhs: Option<Box<Tree>>, rhs: Option<Box<Tree>>) -> Self {
        Self { node, lhs, rhs }
    }
}
