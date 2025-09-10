# NYT Spelling Bee Solver

This is a command-line tool written in Rust that solves the New York Times Spelling Bee puzzle. It takes the seven letters of the puzzle grid as input and finds all valid words from a dictionary file.

## Features
* Word Validation: Identifies words that are at least four letters long, are not proper nouns, contain the required center letter, and are made up exclusively of letters from the puzzle grid.
* Pangram Detection: Automatically finds and highlights "Pangrams"—words that use all seven letters of the puzzle grid.
* Configurable Output: Allows you to format the output into a custom number of columns for better readability.
* Platform Agnostic: Designed to work on Unix-like systems (Linux, macOS) that have a standard dictionary file at `/usr/share/dict/words`.

## How to Compile and Run
### Prerequisites
To compile and run this program, you must have the Rust toolchain installed. The easiest way to get it is by using rustup.

### Using rustc (Rust Compiler)
1. Clone this repository or save the spelling_bee_solver.rs file to your local machine.
2. Navigate to the directory in your terminal.
3. Compile the program using rustc: `rustc spelling_bee_solver.rs`
4. Run the executable:./spelling_bee_solver

### Using cargo (Recommended)
1. Create a new Cargo project in your terminal: `cargo new spelling_bee_solver`
2. Navigate into the new directory: `cd spelling_bee_solver`
3. Copy the spelling_bee_solver.rs code into the src/main.rs file, replacing its contents.
4. Run the program directly using cargo: `cargo run -- c 4 n b d o l e m`

## Usage
The program requires exactly seven letters as arguments, with the first letter being the required center letter. The --columns or -c flag is optional and can be used to control the output format.

```
# Example: Running with 1 column (default)
./spelling_bee_solver n b d o l e m

# Example: Running with 2 columns
./spelling_bee_solver --columns 2 n b d o l e m

# Example: Running with 4 columns using the short flag
./spelling_bee_solver -c 4 n b d o l e m
```

## File Structure
The entire application is contained within a single file for simplicity:`spelling_bee_solver.rs`: The core Rust source code for the Spelling Bee solver.
