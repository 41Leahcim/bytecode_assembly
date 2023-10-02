# Byte Assembly
Byte assembly is a simple programming language compiling to bytecode. This bytecode can then be executed by the compiler.

## Dependencies
Rustc 1.72.1

## Compiler
### Building the compiler
First make sure you have compiled the compiler using `cargo build --release`. You can then find the binary in the target/release/ directory.

### Using the compiler
The compiler only accepts a single input file. The name of this file should end with .basm for binary assembly files or .basmo for the bytecode. If you use the -o flag, the compiler will store the bytecode at the specified path. You can use the -r or --run flag to run the bytecode. The compiler will display an error message if neither was passed.

## Syntax
### Comments
The only supported comment is the multi-line comment. This comment starts with `/*` and ends with `*/` like in most languages. Single line comments are not yet supported, but this may happen in the future.

### Output
To print output, you can use the `out` keyword. It can print strings or any other value. Arguments that aren't strings, will have a new line appended. Strings won't get a new line appended, to support situations where you don't want a new line after output. You can still add a new line yourself, as strings are the only part of the language where escape characters are supported.
Strings support the following escape characters: `\t`, `\n`, `\r`, `\\`, and `\0`.

## Instructions
Instructions also called commands, are tokens that can be executed. A token can take 2 types of arguments, namely registers and numbers. Numbers are always 64-bit signed integers. The registers can only hold numbers as values.
The following table is an overview of all available instructions, their arguments, and what the equivalent in other programming languages. Ra means register a, which could be r0 up to and including r255. Other letters in the arguments mean they could be any valid type of argument.
|Instrution|arguments|in pseudocode|
|-|-|-|
|mov|ra, v|a = v|
|add|ra, b, c|a = b + c|
|sub|ra, b, c|a = b - c|
|mul|ra, b, c|a = b * c|
