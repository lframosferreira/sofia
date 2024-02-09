pub mod parser;
use parser::token::{Token, TokenType};

fn main() -> std::io::Result<()> {
    let content = std::fs::read_to_string("../examples/program01.sf")?;

    let tokens: Vec<TokenType> = vec![];
    let mut index = 0;
    while let Some(c) = content.chars().nth(index) {
        index += 1;
        println!("{}", index);
    }

    Ok(())
}
