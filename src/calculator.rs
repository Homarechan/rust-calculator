use crate::tree;
use std::ops;
use tree::Node::{self, Head};

pub fn calculate(tree: tree::Tree) -> i64 {
    match tree.node {
        Node::Num(n) => n,
        Head => unreachable!(),
        op => {
            let op = match op {
                Node::Plus => ops::Add::add,
                Node::Minus => ops::Sub::sub,
                Node::Mul => ops::Mul::mul,
                Node::Div => ops::Div::div,
                Node::Pow => |a: i64, b: i64| i64::pow(a, b as u32),
                _ => unreachable!(),
            };
            return op(
                calculate(tree.lhs.unwrap().as_ref().clone()),
                calculate(tree.rhs.unwrap().as_ref().clone()),
            );
        }
    }
}
