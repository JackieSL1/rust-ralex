#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    OpenCurly, CloseCurly,
    OpenParen,
    CloseParen,

    Equals,
    Comma,

    Plus,
    Divide,
    Multiply,

    And,
    Or,
    Not,

    Greater,
    GreaterEq,
    Lesser,
    LesserEq,
    
    Project,
    Select,
    Join,
    LeftJoin,
    RightJoin,
    FullJoin,
    Union,
    Intersect,
    Minus,

    Symbol(String),
    String(String),
    Number(String),

    EOF,
}

pub fn get_tokens(chars: std::str::Chars) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = chars.peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '{' => tokens.push(Token::OpenCurly),
            '}' => tokens.push(Token::CloseCurly),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '=' => tokens.push(Token::Equals),
            ',' => tokens.push(Token::Comma),
            '-' => tokens.push(Token::Minus),
            '+' => tokens.push(Token::Plus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '>' => {
                if let Some(next) = chars.peek()  {
                   if *next == '=' {
                        chars.next();
                        tokens.push(Token::GreaterEq);
                        continue;
                   }
                } 
                tokens.push(Token::Greater);
            },
            '<' => {
                if let Some(next) = chars.peek()  {
                   if *next == '=' {
                        chars.next();
                        tokens.push(Token::LesserEq);
                        continue;
                   }
                } 
                tokens.push(Token::Lesser);
            },
            '"' => {
                let mut word: String = "".to_string();

                while let Some(c) = chars.next() {
                    if c == '"' {break};
                    word.push(c);
                }
                // TODO: Add error checking for unterminated strings

                tokens.push(Token::String(word));
            },
            '\'' => {
                let mut word: String = "".to_string();

                while let Some(c) = chars.next() {
                    if c == '\'' {break};
                    word.push(c);
                }
                // TODO: Add error checking for unterminated strings

                tokens.push(Token::String(word));
            },
            _ if c.is_whitespace() => (),
            c if c.is_alphabetic() => {
                let mut word: String = "".to_string();
                word.push(c);

                while let Some(c) = chars.peek() {
                    if !c.is_alphanumeric() {break};
                    word.push(chars.next().unwrap());
                }

                match word.as_str() {
                    "project" => tokens.push(Token::Project),
                    "select" => tokens.push(Token::Select),
                    "join" => tokens.push(Token::Join),
                    "rightJoin" => tokens.push(Token::RightJoin),
                    "leftJoin" => tokens.push(Token::LeftJoin),
                    "fullJoin" => tokens.push(Token::FullJoin),
                    "intersect" => tokens.push(Token::Intersect),
                    "union" => tokens.push(Token::Union),
                    "and" => tokens.push(Token::And),
                    "or" => tokens.push(Token::Or),
                    "not" => tokens.push(Token::Not),
                    _ => tokens.push(Token::Symbol(word)),
                }
            },
            c if c.is_numeric() => {
                let mut word: String = "".to_string();
                word.push(c);

                while let Some(c) = chars.peek() {
                    if !c.is_numeric() {break};
                    word.push(chars.next().unwrap());
                }

                tokens.push(Token::Number(word));
            },
            invalid_char => panic!("Error: unable to parse '{invalid_char}'"),
        }
    }

    // Done reading line
    tokens.push(Token::EOF);
    
    tokens
}
