# Byte Assembly
Byte assembly is a simple programming language compiling to bytecode. This bytecode can then be executed by the compiler.

## Dependencies
Rustc 1.72.1

## Compiler
### Building the compiler
First make sure you have compiled the compiler using `cargo build --release`. You can then find the binary in the target/release/ directory.

### Using the compiler
The compiler only accepts a single input file. The name of this file should end with .basm for binary assembly files or .basmo for the bytecode. If you use the -o flag, the compiler will store the bytecode at the specified path. The compiler will always run the bytecode, also when you pass a .basm file and don't write it to a file.
