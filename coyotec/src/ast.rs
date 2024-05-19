use std::any::Any;
use std::fmt::Display;
use crate::tokens::Location;

pub trait AstNode {
    fn visit(&self);
    fn is_term(&self) -> bool {
        false
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Integer {
    pub value: i64,
}
impl AstNode for Integer {
    fn visit(&self) {
        println!("{}", self.value);
    }

    fn is_term(&self) -> bool {
       true
    }
}

pub struct Float {
    pub value: f64,
}
impl AstNode for crate::ast::Float {
    fn visit(&self) {
        println!("{}", self.value);
    }

    fn is_term(&self) -> bool {
       true
    }

}

pub struct BinOperator {
    pub op: BinOp,
    pub lhs: Box<dyn AstNode>,
    pub rhs: Box<dyn AstNode>,
}

impl AstNode for BinOperator {
    fn visit(&self) {
        self.lhs.visit();
        self.rhs.visit();
        println!("{:?}", self.op);
    }
}

impl BinOperator {
    pub fn new(op: BinOp, lhs: Box<dyn AstNode>, rhs: Box<dyn AstNode>) -> Self {
        Self {
            op,
            lhs,
            rhs,
        }
    }
}

pub struct UnaryOperator {
    pub op: UnaryOp,
    pub expr: Box<dyn AstNode>,
}

impl UnaryOperator {
    pub fn new(op: UnaryOp, expr: Box<dyn AstNode>) -> Self {
        Self {
            op,
            expr,
        }
    }
}

impl AstNode for UnaryOperator {
    fn visit(&self) {
        self.expr.visit();
        println!("{:?}", self.op);
    }

}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[cfg(test)]
mod test {
    use crate::ast::{BinOperator, Integer, AstNode, BinOp};
    use crate::tokens::Location;

    #[test]
    fn test_ast() {
        let loc = Location::new();
        let int = Integer { value: 1 };
        let int2 = Integer { value: 2 };
        let int3 = Integer { value: 3 };
        let int4 = Integer { value: 4 };

        let binop = BinOperator {
            op: BinOp::Mul,
            lhs: Box::new(int),
            rhs: Box::new(int2),
        };

        let sub = BinOperator {
            op: BinOp::Sub,
            lhs: Box::new(binop),
            rhs: Box::new(int4),
        };

        let add = BinOperator {
            op: BinOp::Add,
            lhs: Box::new(sub),
            rhs: Box::new(int3),
        };
        add.visit();
    }
}



