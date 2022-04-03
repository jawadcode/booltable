use std::{
    ops::{Index, Range},
    usize,
};

use derive_more::Display;
use logos::{Logos, SpannedIter};

#[derive(Debug, Display, Logos, PartialEq, Clone, Copy)]
pub enum TK {
    #[token("NOT")]
    #[token("!")]
    #[display(fmt = "NOT")]
    Not,

    #[token("AND")]
    #[token(".")]
    #[display(fmt = "AND")]
    And,

    #[token("OR")]
    #[token("+")]
    #[display(fmt = "OR")]
    Or,

    #[token("XOR")]
    #[token("^")]
    #[token("⊕")]
    #[token("⊻")]
    #[display(fmt = "XOR")]
    Xor,

    #[token("true")]
    #[token("1")]
    #[display(fmt = "True")]
    True,

    #[token("false")]
    #[token("0")]
    #[display(fmt = "False")]
    False,

    #[regex(r"([A-Za-z]|_)([A-Za-z]|_|\d)*")]
    #[display(fmt = "Variable")]
    Var,

    #[token("(")]
    #[display(fmt = "(")]
    LParen,

    #[token(")")]
    #[display(fmt = ")")]
    RParen,

    #[token("=")]
    #[token("->")]
    Equals,

    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    #[error]
    Error,
    Eof,
}
#[derive(Debug, Display, Clone, Copy, PartialEq)]
#[display(fmt = "{}", kind)]
pub struct Token {
    pub kind: TK,
    pub span: Span,
}

impl Token {
    #[inline]
    pub fn text<'input>(&self, input: &'input str) -> &'input str {
        &input[self.span]
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq)]
#[display(fmt = "{}..{}", start, end)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.start..span.end
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[Range::<usize>::from(index)]
    }
}

pub struct Lexer<'input> {
    length: usize,
    logos: SpannedIter<'input, TK>,
    eof: bool,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            length: input.len(),
            logos: TK::lexer(input).spanned(),
            eof: false,
        }
    }
}

impl<'input> Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.logos.next() {
            Some((kind, span)) => Some(Token {
                kind,
                span: span.into(),
            }),
            None if self.eof => None,
            None => {
                self.eof = true;
                Some(Token {
                    kind: TK::Eof,
                    span: (self.length..self.length).into(),
                })
            }
        }
    }
}
