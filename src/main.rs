mod tokenizer;

fn main() {
    // let test_string = r#"
    // Student = {ID, Name, Age, Major
    //     "1", "Alice", 20, "Computer Science"
    //     "2", "Bob", 22, "Physics"
    //     "3", "Charlie", 21, "Mathematics"
    // }
    // "#;
    let test_string = "select id=a Students";

    let tokens: Vec<tokenizer::Token> = tokenizer::get_tokens(test_string.chars());

    println!("Input String:\n{}", test_string);
    println!("{:?}", tokens);
}
