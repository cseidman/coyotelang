use crate::tokens::Location;

pub trait Visitor {
    fn visit(&self);
}

pub struct Integer {
    pub value: i64,
}
impl Visitor for Integer {
    fn visit(&self) {
        println!("{}", self.value);
    }
}

pub struct Float {
    pub value: f64,
}
impl Visitor for crate::ast::Float {
    fn visit(&self) {
        println!("{}", self.value);
    }
}

pub struct BinOperator {
    pub op: BinOp,
    pub lhs: Box<dyn Visitor>,
    pub rhs: Box<dyn Visitor>,
}

impl Visitor for BinOperator {
    fn visit(&self) {
        self.lhs.visit();
        self.rhs.visit();
        println!("{:?}", self.op);
    }
}

impl BinOperator {
    pub fn new(op: BinOp, lhs: Box<dyn Visitor>, rhs: Box<dyn Visitor>) -> Self {
        Self {
            op,
            lhs,
            rhs,
        }
    }
}

pub struct UnaryOperator {
    pub op: BinOp,
    pub expr: Box<dyn Visitor>,
}

impl Visitor for UnaryOperator {
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



#[cfg(test)]
mod test {    use crate::ast::{BinOperator, Integer, Visitor, BinOp};
    use crate::tokens::Location;

    #[test]
    fn test_ast() {
        let loc = Location::new();
        let int = Integer { value: 10 };
        let int2 = Integer { value: 20 };
        let int3 = Integer { value: 3 };

        let binop = BinOperator {
            op: BinOp::Mul,
            lhs: Box::new(int),
            rhs: Box::new(int2),
        };

        let mul = BinOperator {
            op: BinOp::Add,
            lhs: Box::new(binop),
            rhs: Box::new(int3),
        };
        mul.visit();
    }
}

