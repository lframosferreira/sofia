mod parser;

use parser::lexer::tokenize;
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

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
