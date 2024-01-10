#[derive(Debug)]
enum Token {
    OpenCurly,
    CloseCurly,
    Equals,
    Comma,
    Word(String),
    String(String),
    Number(String),
}

// impl std::fmt::Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Token::Word(s) => write!(f, "{}", s),
//             _ => write!(f, "TEST"),
//         }
//     }
// }


fn get_tokens(chars: std::str::Chars) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = chars.peekable();
    
    while let Some(c) = chars.next() {
        println!("{:?}", c);
        match c {
            '{' => tokens.push(Token::OpenCurly),
            '}' => tokens.push(Token::CloseCurly),
            '=' => tokens.push(Token::Equals),
            ',' => tokens.push(Token::Comma),
            '"' => {
                let mut word: String = "".to_string();

                while let Some(c) = chars.next() { // TODO: Possible dangerous unwrap
                    if c == '"' {break};
                    word.push(c);
                }

                tokens.push(Token::String(word));
            },
            _ if c.is_whitespace() => (),
            c if c.is_alphabetic() => {
                let mut word: String = "".to_string();
                word.push(c);

                while chars.peek().unwrap().is_alphabetic() {
                    word.push(chars.next().unwrap());
                }

                tokens.push(Token::Word(word));
            },
            c if c.is_numeric() => {
                let mut word: String = "".to_string();
                word.push(c);

                while chars.peek().unwrap().is_numeric() {
                    word.push(chars.next().unwrap());
                }

                tokens.push(Token::Number(word));
            },
            invalid_char => panic!("Error: unable to parse '{invalid_char}'"),
        }
    }

    tokens
}

fn main() {
    let test_string = r#"
    Student = {ID, Name, Age, Major
        "1", "Alice", 20, "Computer Science"
        "2", "Bob", 22, "Physics"
        "3", "Charlie", 21, "Mathematics"
    }
    "#;

    println!("{:?}", get_tokens(test_string.chars()));
}
