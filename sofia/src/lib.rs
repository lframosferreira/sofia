#![allow(warnings)]
pub mod interpreter;
mod parser;
use std::{os::unix::fs::FileExt, process::exit};

use interpreter::eval::Interpreter;
use parser::lexer::tokenize;
use parser::parser::Parser;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

pub fn interpret(filepath: String) -> std::io::Result<()> {
    let content = std::fs::read_to_string(filepath).expect(
        "Error reading from
input file",
    );

    let tokens = tokenize(content.as_bytes());

    // for token in tokens.clone() {
    //     println!("{:?}", token);
    // }

    let mut parser_object = Parser::new(tokens);
    let tree = parser_object.parse();
    let serialized = serde_json::to_string(&tree).unwrap();
    std::fs::write("output.json", serialized).expect("Unable to write output json file.");

    let mut interp = Interpreter::new();
    interp.eval(tree);

    Ok(())
}
