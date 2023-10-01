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
    Add(u8, Value, Value),
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
    #[arg(short, long)]
    performance: bool,
}

/// Tests the performance of a function
fn test_performance<T: ?Sized, U>(func: fn(&T) -> U, input: &T, action: &str) {
    // Count how many times the function can be called in a second
    let start = Instant::now();
    let mut iterations: u32 = 0;
    while start.elapsed().as_secs() < 1 {
        func(input);
        iterations += 1;
    }

    // Print the result
    eprintln!("{action} {iterations} times per second");
}

fn load_bytecode(args: &Args) -> Vec<Token> {
    // Take the extension
    let Some(extension) = args.file.extension() else {
        panic!("Extension of input file missing");
    };

    // Compile the code to bytecode or load existing bytecode.
    // Only allow basm for code and basmo for bytecode
    match extension.to_str() {
        Some("basm") => {
            // Load the code
            let code = std::fs::read_to_string(&args.file).expect("Input file doesn't exist");

            // Compile the code by splitting it into tokens
            compile::split_tokens(&code).unwrap()
        }
        Some("basmo") => {
            // Read the existing bytecode
            let code = std::fs::read(&args.file).expect("Input file doesn't exist");

            // Convert it to tokens
            postcard::from_bytes::<Vec<Token>>(&code).expect("Invalid basmo input file.")
        }
        _ => {
            // Invalid input file was used
            panic!("Invalid input file!");
        }
    }
}

/// Prints performance
fn print_performance(
    start: Instant,
    parsing: Instant,
    compiling: Instant,
    executing: Instant,
    storing: Instant,
    args: &Args,
    tokens: &[Token],
) {
    // Test execution performance
    test_performance(execute::execute, tokens, "Runs");

    // Test compilation performance for basm files
    if args
        .file
        .extension()
        .is_some_and(|extension| extension == "basm")
    {
        test_performance(
            compile::split_tokens,
            &std::fs::read_to_string(&args.file).unwrap(),
            "Compiles",
        )
    }

    // Print parsing performance
    eprintln!("Parsing args: {}", (parsing - start).as_secs_f64());

    // Print compilation performance, if the code was compiled
    if args
        .file
        .extension()
        .is_some_and(|extension| extension == "basm")
    {
        eprintln!("Compiling   : {}", (compiling - parsing).as_secs_f64());
    }

    // Print runtime performance, if the code was run
    if args.run {
        eprintln!("Executing   : {}", (executing - compiling).as_secs_f64());
    }

    // Print storing performance, if the bytecode was saved
    if args.out.is_some() {
        eprintln!("Storing     : {}", (storing - executing).as_secs_f64());
    }
}

fn main() {
    // Start measuring performance
    let start = Instant::now();

    // Parse the arguments
    let args = Args::parse();
    let parsing = Instant::now();

    // Make sure the code is run or the bytecode is saved
    assert!(
        args.out.is_some() || args.run,
        "The bytecode should be run and/or saved!"
    );

    let tokens = load_bytecode(&args);
    let compiling = Instant::now();

    // Run the code if requested
    if args.run {
        execute::execute(&tokens);
    }
    let executing = Instant::now();

    // Save the bytecode if requested
    if let Some(output) = args.out.as_ref() {
        let file = File::create(output).expect("Failed to create output file");
        let output = BufWriter::new(file);
        postcard::to_io(&tokens, output).expect("Failed to write tokens to output file");
    }
    let storing = Instant::now();

    // Print the tokens if requested
    if args.debug {
        eprintln!("{tokens:?}");
    }

    // Test performance if requested
    if args.performance {
        print_performance(
            start, parsing, compiling, executing, storing, &args, &tokens,
        );
    }
}
