use crate::tokenizer::Token;
use crate::table::Table;
use crate::condition::{Condition, self};
use std::slice::Iter;
use std::iter::Peekable;
use std::collections::HashMap;

pub type List = Vec<String>;

#[derive (Debug)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr>},
    BinaryCond{ left: Box<Expr>, operator: Token, condition: Box<Condition>, right: Box<Expr> },
    Unary { operator: Token, right: Box<Expr> },
    UnaryCond { operator: Token, condition: Box<Condition> ,right: Box<Expr> },
    Grouping(Box<Expr>),
    Literal(Token),
    UnaryList{ operator: Token, list: Box<List> ,right: Box<Expr> },
}


impl Expr {
    pub fn eval<'a>(&'a self, tables: &'a HashMap<String, Table>) -> Option<Table> {
        match self {
            Expr::Binary { left, operator, right } => {
                match operator {
                    Token::Union => {Some(left.eval(&tables).unwrap().union(&right.eval(&tables).unwrap()).unwrap())},
                    Token::Intersect => {Some(left.eval(&tables).unwrap().intersect(&right.eval(&tables).unwrap()).unwrap())},
                    Token::Minus => {Some(left.eval(&tables).unwrap().minus(&right.eval(&tables).unwrap()).unwrap())},
                    Token::Multiply => {Some(left.eval(&tables).unwrap().multiply(&right.eval(&tables).unwrap()).unwrap())},
                    Token::Divide => {Some(left.eval(&tables).unwrap().divide(&right.eval(&tables).unwrap()).unwrap())},
                    _ => panic!("error: can't evaluate {operator:?}"),
                }
            },
            Expr::BinaryCond { left, operator, condition, right } => {
                println!("MATCHING JOIN");
                println!("{:?}, {:?}, {:?}, {:?}", left, operator, condition, right); 
                match operator {
                    Token::Join => {Some(left.eval(&tables).unwrap().join(&condition, &right.eval(&tables).unwrap()).unwrap())},
                    Token::LeftJoin => {Some(left.eval(&tables).unwrap().left_join(&condition, &right.eval(&tables).unwrap()).unwrap())},
                    Token::RightJoin => {Some(left.eval(&tables).unwrap().right_join(&condition, &right.eval(&tables).unwrap()).unwrap())},
                    Token::FullJoin => {Some(left.eval(&tables).unwrap().full_join(&condition, &right.eval(&tables).unwrap()).unwrap())},
                    _ => panic!("error: can't evaluate {operator:?}"),
                }
            },
            Expr::Unary{ operator, .. } => {
                match operator {
                    _ => panic!("error: can't evaluate {operator:?}"),
                }
            },
            Expr::UnaryCond{ operator, condition ,right } => {
                match operator {
                    Token::Select => {Some(right.eval(&tables).unwrap().select(&condition).unwrap())},
                    _ => panic!("error: can't evaluate {operator:?}"),
                }
            },
            Expr::UnaryList{ operator, list, right } => {
                match operator {
                    Token::Project => {Some(right.eval(&tables).unwrap().project(list.to_vec()).unwrap())},
                    _ => panic!("error: can't evaluate {operator:?}"),
                }
            },
            Expr::Literal(token) => {
                match token {
                    Token::Symbol(key) => tables.get(key).cloned(),
                    _ => panic!("error: can't evaluate {token:?} Literal"),
                }
            },
            Expr::Grouping(expr) => expr.eval(&tables),
        }
    }
}

fn expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Box<Expr> { 
    let mut expr = factor(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::Plus | Token::Minus | Token::Union | Token::Intersect => {
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
            },
            Token::Join | Token::LeftJoin | Token::RightJoin | Token::FullJoin => {
                let operator = tokens.next().unwrap().clone();
                let condition = condition::parse(tokens);
                let right = unary(tokens);
                expr = Box::new(Expr::BinaryCond { left: expr, operator, condition, right });
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
Token::Select=> {
                let operator = tokens.next().unwrap().clone();
                let condition = condition::parse(tokens);
                let right = unary(tokens);
                return Box::new(Expr::UnaryCond {operator, condition, right})
            },
            Token::Project => {
                let operator = tokens.next().unwrap().clone();
                let list = Box::new(list(tokens));
                let right = unary(tokens);
                return Box::new(Expr::UnaryList {operator, list, right})
            },
            _ => {},
        }
    }
    primary(tokens)
}

fn primary(tokens: &mut Peekable<Iter<'_, Token>>) -> Box<Expr> { 
    if let Some(token) = tokens.next() {
        match token {
           Token::Symbol(_) | Token::Number(_) | Token::String(_) => Box::new(Expr::Literal(token.clone())),
            Token::OpenParen => { 
                let expr = expr(tokens);
                if tokens.next().unwrap() != &Token::CloseParen {panic!("error: expected ')' after expression")};
                Box::new(Expr::Grouping(expr))
            },
            token => panic!("error: unable to parse {:?}", token),
        }
    } else {
        panic!("error: no more tokens");
    }
}

fn list(tokens: &mut Peekable<Iter<'_, Token>>) -> List { 
    println!("in list");
    let mut result: Vec<String> = Vec::new();
    let Token::Symbol(string) = tokens.next().unwrap() else {
        panic!("error: list expected");
    };
    result.push(string.clone());

    while let Some(&token) = tokens.peek() {
        match token {
            Token::Comma => {
                tokens.next();
                match tokens.next() {
                    Some(Token::Symbol(symbol)) => {result.push(symbol.to_string())},
                    None => panic!("error: unexpected end of list"),
                    _ => panic!("error: list must only contain variables"),
                }
            },
            _ => {
                break;
            }
        }
    }

    result
}

pub fn parse(tokens: &Vec<Token>) -> Box<Expr> {
    let mut iter = tokens.iter().peekable();
    expr(&mut iter)
}
