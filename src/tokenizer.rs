#[derive(Debug)]
pub enum Token {
    OpenCurly,
    CloseCurly,
    Equals,
    Comma,
    Projection,
    Selection,
    Join,
    Union,
    Intersection,
    Subtraction,
    Symbol(String),
    String(String),
    Number(String),
}

pub fn get_tokens(chars: std::str::Chars) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = chars.peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '{' => tokens.push(Token::OpenCurly),
            '}' => tokens.push(Token::CloseCurly),
            '=' => tokens.push(Token::Equals),
            ',' => tokens.push(Token::Comma),
            '-' => tokens.push(Token::Subtraction),
            '"' => {
                let mut word: String = "".to_string();

                while let Some(c) = chars.next() {
                    if c == '"' {break};
                    word.push(c);
                }

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
                    "project" => tokens.push(Token::Projection),
                    "select" => tokens.push(Token::Selection),
                    "join" => tokens.push(Token::Join),
                    "intersect" => tokens.push(Token::Intersection),
                    "union" => tokens.push(Token::Union),
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

    tokens
}
