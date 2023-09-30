use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufWriter, path::PathBuf, time::Instant};
use value::Value;

mod compile;
mod execute;
mod value;

#[derive(Debug, Serialize, Deserialize)]
pub enum Token {
    Comment(String),
    Out(String),
    Mov(u8, Value),
}

#[derive(Debug, Parser)]
struct Args {
    /// The input file
    file: PathBuf,

    /// The output file
    #[arg(short)]
    out: Option<PathBuf>,

    /// Whether the binary should be run
    #[arg(short, long)]
    run: bool,

    /// Whether the tokens should be printed
    #[arg(short, long)]
    debug: bool,

    /// Whether to print performance.
    /// If you write the output of your code to a file, you can also easily
    /// see the performance per second.
    #[arg(short, long)]
    performance: bool,
}

fn test_performance<T: ?Sized, U>(func: fn(&T) -> U, input: &T, action: &str) {
    let start = Instant::now();
    let mut iterations: u32 = 0;
    while start.elapsed().as_secs() < 1 {
        func(input);
        iterations += 1;
    }
    eprintln!("{action} {iterations} times per second");
}

fn main() {
    let start = Instant::now();

    let args = Args::parse();
    let parsing = Instant::now();

    if args.out.is_none() && !args.run {
        panic!("The bytecode should be run and/or saved!");
    }

    let Some(extension) = args.file.extension() else {
        panic!("Extension of input file missing");
    };

    let tokens = if extension == "basm" {
        let code = std::fs::read_to_string(&args.file).expect("Input file doesn't exist");
        if args.performance {
            test_performance(compile::split_tokens, code.as_str(), "Compiles");
        }

        compile::split_tokens(&code)
    } else if extension == "basmo" {
        let code = std::fs::read(&args.file).expect("Input file doesn't exist");
        postcard::from_bytes::<Vec<Token>>(&code).expect("Invalid basmo input file.")
    } else {
        panic!("Invalid input file!");
    };
    let compiling = Instant::now();

    if args.run {
        execute::execute(&tokens);
    }
    let executing = Instant::now();

    if let Some(output) = args.out.as_ref() {
        let file = File::create(output).expect("Failed to create output file");
        let output = BufWriter::new(file);
        postcard::to_io(&tokens, output).expect("Failed to write tokens to output file");
    }
    let storing = Instant::now();

    if args.debug {
        eprintln!("{tokens:?}");
    }

    if args.performance {
        test_performance(execute::execute, &tokens, "Runs");
        println!("Parsing args: {}", (parsing - start).as_secs_f64());
        if extension == "basm" {
            println!("Compiling   : {}", (compiling - parsing).as_secs_f64());
        }
        if args.run {
            println!("Executing   : {}", (executing - compiling).as_secs_f64());
        }
        if args.out.is_some() {
            println!("Storing     : {}", (storing - executing).as_secs_f64());
        }
    }
}
