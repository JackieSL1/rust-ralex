use crate::tokenizer::Token;
use std::{cell::RefCell, rc::Rc};
use std::slice::Iter;
use std::iter::Peekable;

#[derive (Debug)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr>},
    Unary { operator: Token, right: Box<Expr> },
    Literal(String),
}

impl Expr {
    pub fn eval(&self) -> i32 {
       match self {
            Expr::Binary { left, operator, right } => {
                match operator {
                    Token::Plus => {left.eval() + right.eval()},
                    Token::Minus => {left.eval() - right.eval()},
                    Token::Multiply => {left.eval() * right.eval()},
                    Token::Divide => {left.eval() / right.eval()},
                    _ => panic!("error: can't evaluate {operator:?}"),
                }
            },
            Expr::Unary{ operator, right } => {
                match operator {
                    Token::Minus => {- right.eval()},
                    _ => panic!("error: can't evaluate {operator:?}"),
                }
            },
            Expr::Literal(string) => string.parse::<i32>().unwrap(),
       }
    }
}

fn term(tokens: &mut Peekable<Iter<'_, Token>>) -> Box<Expr> { 
    let mut expr = factor(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::Plus | Token::Minus=> {
                let operator = tokens.next().unwrap().clone();
                let right = factor(tokens);
                expr = Box::new(Expr::Binary {left: expr, operator, right}); 
            }
            _ => break,
        }
    }

    expr
}

fn factor(tokens: &mut Peekable<Iter<'_, Token>>) -> Box<Expr> { 
    let mut expr = unary(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::Multiply | Token::Divide => {
                let operator = tokens.next().unwrap().clone();
                let right = unary(tokens);
                let new_expr = Expr::Binary { left: expr, operator, right};
                expr = Box::new(new_expr);
            }
            _ => break,
        }
    }

    expr
}

fn unary(tokens: &mut Peekable<Iter<'_, Token>>) -> Box<Expr> { 
    if let Some(token) = tokens.peek() {
        match token {
            Token::Minus => {
                let operator = tokens.next().unwrap().clone();
                let right = unary(tokens);
                return Box::new(Expr::Unary {operator, right})
            },
            _ => {},
        }
    }
    primary(tokens)
}

fn primary(tokens: &mut Peekable<Iter<'_, Token>>) -> Box<Expr> { 
    if let Some(token) = tokens.next() {
        match token {
            Token::Number(num) => Box::new(Expr::Literal(num.to_string())),
            token => panic!("error: unable to parse {:?}", token),
        }
    } else {
        panic!("error: no more tokens");
    }
}

pub fn parse(tokens: &Vec<Token>) -> Box<Expr> {
    let mut iter = tokens.iter().peekable();
    term(&mut iter)
}
