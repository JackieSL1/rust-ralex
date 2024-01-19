mod tokenizer;
mod parser;
mod table;
mod condition;

use std::io::{self, Write};
use std::collections::HashMap;
use crate::tokenizer::*;
use crate::table::Table;


fn process_table_tokens(tokens: Vec<Vec<Token>>) -> Result<Vec<Vec<Token>>, String> {
    let mut result: Vec<Vec<Token>> = Vec::new();
    let mut tokens = tokens.iter()
        .flatten()
        .skip_while( |token| **token != Token::OpenCurly )
        .peekable();

    let mut row: Vec<Token> = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
            Token::CloseCurly => {return Ok(result);},
            Token::Comma | Token::EOF | Token::OpenCurly => {},
            Token::Symbol(_) | Token::Number(_) | Token::String(_)=> {
                row.push(token.clone());
                if *tokens.peek().unwrap() != &Token::Comma {
                    result.push(row.clone());
                    row.clear();
                }
            },
            _ => { return Err(format!("error: unable to parse token {:?} while processing table", token)); },
        };
    }
    Ok(result)
}

fn build_table(mut tokens: Vec<Vec<Token>>) -> Table {
    if tokens.len() < 2 {
        panic!("error: table must have at least 2 rows"); 
    }
    let mut table = Table { 
        rows: Vec::new(),
        types: tokens[1].iter_mut().map( |token| {
            match token {
               Token::Number(_) => "Integer".to_string(),
               Token::String(_) => "String".to_string(),
               _ => panic!("error: unable to identify table types"),
            }
            }).collect::<Vec<String>>(),
    };
    let width = tokens[0].len();
    for row in tokens.iter_mut() {
        if row.len() != width {
            panic!("error: all table rows must be same length");// TODO: shouldn't panic
        }
        table.rows.push(row.iter_mut()
            .enumerate().map( |(i, token)| {
                println!("{i}: {:?}", table.types[i]);
                match token {
                    Token::Symbol(string) => string.clone(),
                    Token::Number(string) => {
                        if table.types[i] != "Integer" {panic!("error: cannot store non-Integer values in Integer coloumn")};
                        string.clone()
                    },
                    Token::String(string) => {
                        if table.types[i] != "String" {panic!("error: cannot store non-String values in String coloumn")};
                        string.clone()
                    },
                    _ => panic!("error: unable to build row"),
                }
            })
            .collect()
        );
    }

    println!("Built new table: {table:?}");

    table
}

fn main() {
    // let test_string = r#"
    // Student = {ID, Name, Age, Major
    //     "1", "Alice", 20, "Computer Science"
    //     "2", "Bob", 22, "Physics"
    //     "3", "Charlie", 21, "Mathematics"
    // }
    // "#;


    let mut tables: HashMap<String, Table> = HashMap::new();
    tables.insert("a".to_string(), Table::new(vec!["test".to_string()]));

    loop {
        print!("> ");
        io::stdout().flush().expect("error: unable to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        let mut tokens: Vec<Token> = get_tokens(input.chars());
        //println!("Tokens: {tokens:?}");
        // TODO: Handle newline
        if tokens.len() == 2 {
            if let Token::Symbol(command) = &tokens[0] {
                match command.to_lowercase().as_str() {
                    "quit" | "exit" => {
                        println!("Exiting... Have a nice day!");
                        break;
                    },
                    "help" | "h" => {
                        println!("Need help?");
                    },
                    _ => {},
                }
            }
        }
        if tokens[0] == Token::EOF {
            println!("Exiting... Have a nice day!");
            break;
        } else if tokens[1] == Token::Equals {
            let Token::Symbol(table_name) = tokens[0].clone() else {break;};
            let mut table_tokens = vec![tokens.clone()];

            while tokens[0] != Token::EOF {
                let mut input = String::new();
                print!("\t");
                io::stdout().flush().expect("error: unable to flush stdout");
                io::stdin().read_line(&mut input).expect("error: unable to read user input");
                tokens = get_tokens(input.chars());
                table_tokens.push(tokens.clone());
                //println!("Tokens: {tokens:?}");
            }

            let result = process_table_tokens(table_tokens);
            match result {
                Ok(toks) => {tables.insert(table_name.to_string(), build_table(toks));},
                Err(msg) => {eprintln!("{msg}");},
            };

        } else {
            tokens.pop(); // Remove EOF

            let tree = parser::parse(&tokens);

            let result = tree.eval(&tables);
            match result {
                Some(table) => println!("{table}"),
                None => println!("No table found"),
            };
        }
    }

    //Table::new(vec!["1".to_string(), "2".to_string()]);
}


