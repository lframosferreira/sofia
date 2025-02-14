use clap::{Args, Parser, Subcommand, ValueEnum};
use sofia::interpret;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    // Runs the interpreter
    Run(RunCommandConfig),

    // Runs the compiler
    Compile(CompileCommandConfig),
}

#[derive(Args)]
struct RunCommandConfig {
    // Path to the input file to be interpreted
    input_file_path: String,
}

#[derive(Args)]
struct CompileCommandConfig {
    // Path to the input file to be compiled
    input_file_path: String,

    // Compilation target
    #[arg(long)]
    target: CompilationTarget,
}

#[derive(ValueEnum, Clone, Debug)]
enum CompilationTarget {
    Asm,
    Python,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Run(run_config) => interpret(run_config.input_file_path).expect("error"),
        _ => panic!("compiler not implemented yet"),
    }
}
