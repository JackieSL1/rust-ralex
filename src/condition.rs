use crate::tokenizer::Token;
use std::slice::Iter;
use std::iter::Peekable;
use std::collections::HashMap;

#[derive (Debug)]
pub enum Condition {
    Binary { left: Box<Condition>, operator: Token, right: Box<Condition>},
    Unary { operator: Token, right: Box<Condition> },
    Grouping(Box<Condition>),
    Literal(Token),
}

impl Condition {
    pub fn eval<'a>(&'a self, row_lookup: &'a HashMap<String, String>) -> String {
        match self {
            Condition::Binary { left, operator, right } => {
                match operator {
                    Token::Equals => {(left.eval(row_lookup) == right.eval(row_lookup)).to_string()},
                    Token::Greater => {(left.eval(row_lookup) > right.eval(row_lookup)).to_string()},
                    Token::GreaterEq => {(left.eval(row_lookup) >= right.eval(row_lookup)).to_string()},
                    Token::Lesser => {(left.eval(row_lookup) < right.eval(row_lookup)).to_string()},
                    Token::LesserEq => {(left.eval(row_lookup) <= right.eval(row_lookup)).to_string()},
                    Token::And => {(left.eval(row_lookup).parse().unwrap() && right.eval(row_lookup).parse().unwrap()).to_string()},
                    Token::Or => {(left.eval(row_lookup).parse().unwrap() || right.eval(row_lookup).parse().unwrap()).to_string()},
                    Token::Comma=> {
                        let result = left.eval(row_lookup) + "," + &right.eval(row_lookup);
                        result
                    },
                    _ => panic!("error: can't evaluate {operator:?}"),
                }
            },
            Condition::Unary{ operator, right } => {
                match operator {
                    Token::Not => {(!right.eval(row_lookup).parse::<bool>().unwrap()).to_string()},
                    _ => panic!("error: can't evaluate {operator:?}"),
                }
            },
            Condition::Literal(token) => {
                match token {
                    Token::Symbol(key) => row_lookup.get(key).unwrap().to_string(),
                    Token::Number(val) | Token::String(val) => val.to_string(),
                    _ => panic!("error: can't evaluate {token:?} Literal"),
                }
            },
            Condition::Grouping(condition) => condition.eval(&row_lookup),
        }
    }
}

pub fn parse(tokens: &mut Peekable<Iter<'_, Token>>) -> Box<Condition> { 
    let mut condition = comparison(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::And | Token::Or => {
                let operator = tokens.next().unwrap().clone();
                let right = comparison(tokens);
                condition = Box::new(Condition::Binary {left: condition, operator, right}); 
            }
            _ => break,
        }
    }

    condition
}

fn comparison(tokens: &mut Peekable<Iter<'_, Token>>) -> Box<Condition> { 
    let mut condition = unary(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::Greater | Token::GreaterEq | Token::Lesser | Token::LesserEq | Token::Equals | Token::Comma => {
                let operator = tokens.next().unwrap().clone();
                let right = unary(tokens);
                let new_cond = Condition::Binary { left: condition, operator, right};
                condition = Box::new(new_cond);
            }
            _ => break,
        }
    }

    condition
}

fn unary(tokens: &mut Peekable<Iter<'_, Token>>) -> Box<Condition> { 
    if let Some(token) = tokens.peek() {
        match token {
            Token::Minus | Token::Comma | Token::Not => {
                let operator = tokens.next().unwrap().clone();
                let right = unary(tokens);
                return Box::new(Condition::Unary {operator, right})
            },
            _ => {},
        }
    }
    primary(tokens)
}

fn primary(tokens: &mut Peekable<Iter<'_, Token>>) -> Box<Condition> { 
    if let Some(token) = tokens.next() {
        match token {
            Token::Number(_) | Token::Symbol(_) | Token::String(_) => Box::new(Condition::Literal(token.clone())),
            Token::OpenParen => { 
                let expr = parse(tokens);
                if tokens.next().unwrap() != &Token::CloseParen {panic!("error: expected ')' after expression")};
                Box::new(Condition::Grouping(expr))
            },
            token => panic!("error: unable to parse {:?}", token),
        }
    } else {
        panic!("error: no more tokens");
    }
}
