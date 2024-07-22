mod parser;

use parser::lexer::tokenize;

fn main() -> std::io::Result<()> {
    let content = std::fs::read_to_string("../examples/program01.sf").expect(
        "Error reading from
input file",
    );

    let tokens = tokenize(content);

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
