mod tokenizer;
mod parser;
mod table;

use std::io::{self, Write};
use std::collections::HashMap;
use crate::parser::Expr;
use crate::table::Table;

fn main() {
    // let test_string = r#"
    // Student = {ID, Name, Age, Major
    //     "1", "Alice", 20, "Computer Science"
    //     "2", "Bob", 22, "Physics"
    //     "3", "Charlie", 21, "Mathematics"
    // }
    // "#;
    // let test_string = "select id=a Students";

    // let tokens: Vec<tokenizer::Token> = tokenizer::get_tokens(test_string.chars());

    // println!("Input String:\n{}", test_string);
    // println!("{:?}", tokens);
    //
    
    let mut tables: HashMap<String, Table> = HashMap::new();
    tables.insert("a".to_string(), Table::new(vec!["test".to_string()]));

    loop {
        print!("> ");
        io::stdout().flush().expect("error: unable to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        let mut tokens: Vec<tokenizer::Token> = tokenizer::get_tokens(input.chars());
        println!("Tokens: {tokens:?}");
        if tokens[0] == tokenizer::Token::EOF {
            break;
        }
        tokens.pop(); // Remove EOF

        let tree = parser::parse(&tokens);
        println!("Tree: {tree:?}");

        tree.eval(&tables);
    }

    //Table::new(vec!["1".to_string(), "2".to_string()]);
}
