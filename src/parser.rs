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

fn term(tokens: &mut Peekable<Iter<'_, Token>>) -> Box<Expr> { 
    let mut expr = factor(tokens);
    println!("expr = {expr:?}");

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
    println!("DONE");

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

    primary(tokens)}

fn unary(tokens: &mut Peekable<Iter<'_, Token>>) -> Box<Expr> { 
    if let Some(token) = tokens.peek() {
        match token {
            Token::Minus => {
                let operator = tokens.next().unwrap().clone();
                let right = unary(tokens);
                return Box::new(Expr::Unary {operator, right})
            },
            _ => (),
        }
    }
    println!("returning primary");
    primary(tokens)
}

fn primary(tokens: &mut Peekable<Iter<'_, Token>>) -> Box<Expr> { 
    if let Some(token) = tokens.next() {
        println!("MATCH: {token:?}");
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
