use crate::{
    parser::{self, BinOp, Expr, SpanExpr},
    vm::Op,
};

pub struct Equation<'input> {
    pub inputs: Vec<&'input str>,
    pub lhs: Vec<Op>,
    pub output: &'input str,
}

pub struct Compiler<'input> {
    equation: parser::Equation<'input>,
}

impl<'input> Compiler<'input> {
    pub fn new(equation: parser::Equation<'input>) -> Self {
        Self { equation }
    }

    pub fn compile(&self) -> Equation {
        let mut lhs = vec![];
        Self::compile_expr(&mut lhs, &self.equation.lhs);

        Equation {
            inputs: self.equation.inputs.clone(),
            lhs,
            output: self.equation.output,
        }
    }

    fn compile_expr(ops: &mut Vec<Op>, expr: &SpanExpr) {
        match &expr.node {
            Expr::Bool(b) => ops.push(Op::Push(*b)),
            Expr::Var(v) => ops.push(Op::Load(*v)),
            Expr::Not(e) => {
                Self::compile_expr(ops, e);
                ops.push(Op::Not);
            }
            Expr::BinOp { op, lhs, rhs } => {
                Self::compile_expr(ops, rhs);
                Self::compile_expr(ops, lhs);

                ops.push(match op {
                    BinOp::And => Op::And,
                    BinOp::Or => Op::Or,
                    BinOp::Xor => Op::Xor,
                });
            }
        }
    }
}
