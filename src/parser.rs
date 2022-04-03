use crate::lexer::{Lexer, Span, Token, TK};
use derive_more::Display;
use std::{collections::HashMap, fmt, iter::Peekable};

#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum BinOp {
    #[display(fmt = "AND")]
    And,
    #[display(fmt = "OR")]
    Or,
    #[display(fmt = "XOR")]
    Xor,
}

impl From<TK> for BinOp {
    fn from(tk: TK) -> Self {
        match tk {
            TK::And => BinOp::And,
            TK::Or => BinOp::Or,
            TK::Xor => BinOp::Xor,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Display, Clone, PartialEq)]
#[display(fmt = "{}", node)]
pub struct Spanned<T>
where
    T: fmt::Debug + fmt::Display + Clone + PartialEq,
{
    pub span: Span,
    pub node: T,
}

pub type SpanExpr = Spanned<Expr>;
pub type Boxode = Box<SpanExpr>;

#[derive(Debug, Display, Clone, PartialEq)]
pub enum Expr {
    #[display(fmt = "{}", _0)]
    Bool(bool),
    #[display(fmt = "{}", _0)]
    Var(usize),
    #[display(fmt = "(NOT {})", _0)]
    Not(Boxode),
    #[display(fmt = "({} {} {})", op, lhs, rhs)]
    BinOp { op: BinOp, lhs: Boxode, rhs: Boxode },
}

pub struct Parser<'input> {
    input: &'input str,
    lexer: Peekable<Lexer<'input>>,
    variables: HashMap<&'input str, usize>,
    counter: usize,
}

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedToken { expected: String, got: Token },
    UnexpectedEof(Token),
}

pub type ParseResult<T> = Result<T, SyntaxError>;

#[derive(Debug, Display)]
#[display(
    fmt = "Equation:\ninputs = {:#?}\nlhs = {}\noutput = {}",
    inputs,
    lhs,
    output
)]
pub struct Equation<'input> {
    pub inputs: Vec<&'input str>,
    pub lhs: SpanExpr,
    pub output: &'input str,
}

macro_rules! spanned {
    ($span:expr, $node:expr) => {
        Ok(Spanned {
            span: $span.into(),
            node: $node,
        })
    };
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            lexer: Lexer::new(input).peekable(),
            variables: HashMap::new(),
            counter: 0,
        }
    }

    pub fn parse_equation(&mut self) -> ParseResult<Equation> {
        let lhs = self.parse_expr()?;
        self.consume(TK::Equals)?;
        let output = self.expect(TK::Var)?.text(self.input);

        let mut variables = self
            .variables
            .clone()
            .into_iter()
            .collect::<Vec<(&str, usize)>>();
        variables.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let inputs = variables
            .into_iter()
            .map(|var| var.0)
            .collect::<Vec<&str>>();

        Ok(Equation {
            inputs,
            lhs,
            output,
        })
    }

    fn parse_expr(&mut self) -> ParseResult<SpanExpr> {
        let mut lhs = match self.peek() {
            t @ TK::True | t @ TK::False => self.parse_bool(t),
            TK::Var => self.parse_var(),
            TK::Not => self.parse_not(),
            TK::LParen => self.parse_group(),

            _ => {
                let token = self.next()?;
                return Err(SyntaxError::UnexpectedToken {
                    expected: "boolean expression".to_string(),
                    got: token,
                });
            }
        }?;

        loop {
            let op = match self.peek() {
                op @ TK::And | op @ TK::Or | op @ TK::Xor => BinOp::from(op),
                TK::RParen | TK::Equals | TK::Eof => break,
                _ => {
                    let token = self.next()?;
                    return Err(SyntaxError::UnexpectedToken {
                        expected: "AND, OR, XOR or )".to_string(),
                        got: token,
                    });
                }
            };
            self.advance();

            let rhs = self.parse_expr()?;
            lhs = Spanned {
                span: (lhs.span.start..rhs.span.end).into(),
                node: Expr::BinOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            };
        }

        Ok(lhs)
    }

    fn parse_bool(&mut self, t: TK) -> ParseResult<SpanExpr> {
        let token = self.next().unwrap();
        spanned!(token.span, Expr::Bool(t == TK::True))
    }

    fn parse_var(&mut self) -> ParseResult<SpanExpr> {
        let token = self.next().unwrap();
        let text = token.text(self.input);
        let index = self.insert_var(text);

        spanned!(token.span, Expr::Var(index))
    }

    fn parse_not(&mut self) -> ParseResult<SpanExpr> {
        let not_token = self.next().unwrap();
        let expr = Box::new(self.parse_expr()?);

        spanned!(not_token.span.start..expr.span.end, Expr::Not(expr))
    }

    fn parse_group(&mut self) -> ParseResult<SpanExpr> {
        let lp_token = self.next().unwrap();
        let expr = self.parse_expr()?;
        let rp_token = self.expect(TK::RParen)?;

        spanned!(lp_token.span.start..rp_token.span.end, expr.node)
    }

    fn next(&mut self) -> ParseResult<Token> {
        self.lexer.next().ok_or_else(|| {
            let len = self.input.len();
            SyntaxError::UnexpectedEof(Token {
                kind: TK::Eof,
                span: (len..len).into(),
            })
        })
    }

    fn peek(&mut self) -> TK {
        self.lexer.peek().map(|token| token.kind).unwrap_or(TK::Eof)
    }

    fn advance(&mut self) {
        self.lexer.next().unwrap();
    }

    fn consume(&mut self, expected: TK) -> ParseResult<()> {
        let token = self.next()?;
        if token.kind != expected {
            Err(SyntaxError::UnexpectedToken {
                expected: expected.to_string(),
                got: token,
            })
        } else {
            Ok(())
        }
    }

    fn expect(&mut self, expected: TK) -> ParseResult<Token> {
        let token = self.next()?;
        if token.kind != expected {
            Err(SyntaxError::UnexpectedToken {
                expected: expected.to_string(),
                got: token,
            })
        } else {
            Ok(token)
        }
    }

    fn insert_var(&mut self, key: &'input str) -> usize {
        let a = self.variables.entry(key).or_insert_with(|| {
            self.counter += 1;
            self.counter - 1
        });

        *a
    }
}
