use crate::tokenizer::Token;
use crate::table::Table;
use std::slice::Iter;
use std::iter::Peekable;
use std::collections::HashMap;

// struct parser {
//     tables: HashMap<String, Table>
// }

#[derive (Debug)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr>},
    Unary { operator: Token, right: Box<Expr> },
    Grouping(Box<Expr>),
    Literal(Token),
}

impl Expr {
    pub fn eval(&self, tables: &HashMap<String, Table>) -> i32 {
        match self {
            Expr::Binary { left, operator, right } => {
                match operator {
                    Token::Plus => {left.eval(&tables) + right.eval(&tables)},
                    Token::Minus => {left.eval(&tables) - right.eval(&tables)},
                    Token::Multiply => {left.eval(&tables) * right.eval(&tables)},
                    Token::Divide => {left.eval(&tables) / right.eval(&tables)},
                    _ => panic!("error: can't evaluate {operator:?}"),
                }
            },
            Expr::Unary{ operator, right } => {
                match operator {
                    Token::Minus => {- right.eval(&tables)},
                    _ => panic!("error: can't evaluate {operator:?}"),
                }
            },
            Expr::Literal(token) => {
                match token {
                    Token::Number(num) => {
                        println!("{}", num.parse::<i32>().unwrap());
                        1
                    },
                    Token::Symbol(key) => {
                        match tables.get(key) {
                            Some(table) => println!("{table}"),
                            None => println!("No table with name {key}"),
                        };
                        0
                    },
                    _ => panic!("error: can't evaluate {token:?} Literal"),
                }
            },
            Expr::Grouping(expr) => expr.eval(&tables),
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
            Token::Number(_) | Token::Symbol(_) => Box::new(Expr::Literal(token.clone())),
            Token::OpenParen => { 
                let expr = term(tokens);
                if tokens.next().unwrap() != &Token::CloseParen {panic!("error: expected ')' after expression")};
                Box::new(Expr::Grouping(expr))
            },
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
