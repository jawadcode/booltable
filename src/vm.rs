use std::fmt;

use derive_more::Display;

use crate::compiler::Equation;

pub type Value = bool;

#[derive(Debug, Display, Clone, Copy)]
pub enum Op {
    #[display(fmt = "PUSH {}", _0)]
    Push(Value),
    #[display(fmt = "LOAD {}", _0)]
    Load(usize),
    #[display(fmt = "NOT")]
    Not,
    #[display(fmt = "AND")]
    And,
    #[display(fmt = "OR")]
    Or,
    #[display(fmt = "XOR")]
    Xor,
}

pub struct VM<'input> {
    equation: Equation<'input>,
    stack: Vec<Value>,
}

#[derive(Debug)]
pub struct TruthTable<'input> {
    pub input_names: Vec<&'input str>,
    pub inputs: Vec<Vec<bool>>,
    pub output_name: &'input str,
    pub outputs: Vec<bool>,
}

impl<'input> fmt::Display for TruthTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output_length = self.output_name.len();
        let lengths: Vec<usize> = self.input_names.iter().map(|n| n.len()).collect();
        writeln!(
            f,
            "| {} | {} |",
            join(&self.input_names, " | "),
            self.output_name
        )?;
        writeln!(
            f,
            "|{}|{}|",
            lengths
                .iter()
                .map(|x| "-".repeat(*x + 2))
                .collect::<Vec<_>>()
                .join("|"),
            "-".repeat(output_length + 2)
        )?;

        let rows: Vec<String> = self
            .inputs
            .iter()
            .map(|r| {
                format!(
                    "| {} |",
                    r.iter()
                        .zip(&lengths)
                        .map(|(a, b)| format!("{:<b$}", if *a { 1 } else { 0 }))
                        .collect::<Vec<_>>()
                        .join(" | ")
                )
            })
            .zip(&self.outputs)
            .map(|(i, o)| format!("{} {:<output_length$} |", i, if *o { 1 } else { 0 }))
            .collect();
        writeln!(f, "{}", join(&rows, "\n"),)
    }
}

fn join<T: ToString>(things: &[T], sep: &str) -> String {
    things
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(sep)
}

impl<'input> VM<'input> {
    pub fn new(equation: Equation<'input>) -> Self {
        Self {
            equation,
            stack: vec![],
        }
    }

    fn exec(&mut self, inputs: &[Value]) -> Value {
        macro_rules! binop {
            ($op:tt) => {{
                let lhs = self.pop();
                let rhs = self.pop();
                self.stack.push(lhs $op rhs);
            }};
        }

        for ip in 0..self.equation.lhs.len() {
            match self.equation.lhs[ip] {
                Op::Push(v) => self.stack.push(v),
                Op::Load(i) => self.stack.push(inputs[i]),
                Op::Not => {
                    let operand = self.pop();
                    self.stack.push(!operand);
                }
                Op::And => binop!(&&),
                Op::Or => binop!(||),
                Op::Xor => binop!(^),
            }
        }

        self.pop()
    }

    pub fn gen(&mut self) -> TruthTable<'input> {
        let length = self.equation.inputs.len();
        let num_rows = 1 << length;
        let inputs: Vec<Vec<bool>> = (0..num_rows).map(|i| usize_to_bools(i, length)).collect();
        let outputs: Vec<bool> = inputs.iter().map(|inputs| self.exec(inputs)).collect();

        TruthTable {
            input_names: self.equation.inputs.clone(),
            inputs,
            output_name: self.equation.output,
            outputs,
        }
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}

fn usize_to_bools(num: usize, digits: usize) -> Vec<bool> {
    (1..=digits)
        .map(|i| (num >> (digits - i)) & 1 == 1)
        .collect()
}

#[test]
fn test() {
    let test = 0b101010;
    println!("{:#b} {:?}", test, usize_to_bools(test, 6));
}
