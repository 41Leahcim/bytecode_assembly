use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufWriter, path::PathBuf, time::Instant};

mod compile;
mod execute;

#[derive(Debug, Serialize, Deserialize)]
pub enum Token {
    Comment(String),
    Out(String),
}

#[derive(Debug, Parser)]
struct Args {
    /// The input file
    file: PathBuf,

    /// The output file
    #[arg(short)]
    out: Option<PathBuf>,
}

fn main() {
    let start = Instant::now();

    let args = Args::parse();
    let parsing = Instant::now();

    let Some(extension) = args.file.extension() else {
        panic!("Extension of input file missing");
    };

    let tokens = if extension == "basm" {
        let code = std::fs::read_to_string(args.file).expect("Input file doesn't exist");

        compile::split_tokens(&code)
    } else if extension == "basmo" {
        let code = std::fs::read(args.file).expect("Input file doesn't exist");
        postcard::from_bytes::<Vec<Token>>(&code).expect("Invalid basmo input file.")
    } else {
        panic!("Invalid input file!");
    };
    let compiling = Instant::now();

    execute::execute(&tokens);
    let executing = Instant::now();

    if let Some(output) = args.out {
        let file = File::create(output).expect("Failed to create output file");
        let output = BufWriter::new(file);
        postcard::to_io(&tokens, output).expect("Failed to write tokens to output file");
    }
    let storing = Instant::now();

    println!("Parsing args: {}", (parsing - start).as_secs_f64());
    println!("Compiling: {}", (compiling - parsing).as_secs_f64());
    println!("Executing: {}", (executing - compiling).as_secs_f64());
    println!("Storing: {}", (storing - executing).as_secs_f64());
}
