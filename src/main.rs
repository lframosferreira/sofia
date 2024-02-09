fn main() -> std::io::Result<()> {
    let content = std::fs::read_to_string("../examples/program01.sf")?;
    println!("{}", content);
    Ok(())
}
