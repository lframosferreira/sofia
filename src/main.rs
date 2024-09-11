mod parser;
use std::process::exit;

use parser::lexer::tokenize;
use parser::parser::Parser;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <parameter>", args[0]);
        std::process::exit(1);
    }
    let content = std::fs::read_to_string(String::from(args[1].clone())).expect(
        "Error reading from
input file",
    );

    let tokens = tokenize(content.as_bytes());

    for token in tokens.clone() {
        println!("{:?}", token);
    }

    let mut parser_object = Parser::new(tokens);
    let tree = parser_object.parse();
    println!("{:#?}", tree);

    Ok(())
}
