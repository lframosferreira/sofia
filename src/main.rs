#![allow(warnings)]
mod parser;
mod interpreter;
use std::{os::unix::fs::FileExt, process::exit};

use interpreter::{eval, type_checker};
use parser::lexer::tokenize;
use parser::parser::Parser;
use serde::{Deserialize, Serialize};
use serde_json;
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
    let serialized = serde_json::to_string(&tree).unwrap();
    std::fs::write("output.json", serialized).expect("UNable to write output json file.");
    // println!("{:#?}", tree);

    Ok(())
}
