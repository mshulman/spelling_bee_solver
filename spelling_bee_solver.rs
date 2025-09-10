// A Rust program to solve the NYT Spelling Bee puzzle.
//
// To compile and run this program, save it as `spelling_bee_solver.rs` and use
// the Rust compiler:
//
//   rustc spelling_bee_solver.rs
//   ./spelling_bee_solver --columns 2 a b c d e f g
//
// Or, if using Cargo:
//
//   cargo new spelling_bee_solver
//   cd spelling_bee_solver
//   // Copy this code into src/main.rs
//   cargo run -- --columns 2 a b c d e f g

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

// Main function, the entry point of the program.
fn main() -> io::Result<()> {
    const COLUMN_SPACING: usize = 3;

    let mut letters: Vec<String> = Vec::new();
    let mut num_columns: usize = 1;

    let mut args_iter = env::args().skip(1);
    while let Some(arg) = args_iter.next() {
        if arg == "-c" || arg == "--columns" {
            if let Some(n_str) = args_iter.next() {
                num_columns = n_str.parse().unwrap_or(1);
                if num_columns == 0 {
                    num_columns = 1;
                }
		if num_columns > 9 {
		    num_columns = 9;
		}
            }
        } else {
            letters.push(arg);
        }
    }

    if letters.len() != 7 {
        eprintln!("Error: Please provide exactly 7 letters as arguments, with or without a --columns flag.");
        eprintln!("Usage: spelling_bee_solver [-c <num_columns>] <center_letter> <other_letter_1> ... <other_letter_6>");
        return Ok(());
    }

    let center_letter: &str = &letters[0];
    let valid_letters: HashSet<char> = letters.iter()
        .filter_map(|arg| arg.chars().next())
        .filter_map(|c| c.to_lowercase().next())
        .collect();

    let dictionary_path = "/usr/share/dict/words";
    let file = File::open(dictionary_path)?;
    let reader = io::BufReader::new(file);

    // Use a single iterator chain to process and collect all valid words.
    let all_valid_words: Vec<String> = reader.lines()
        .filter_map(|line_result| line_result.ok())
        .filter(|word| {
            // Criterion 2: Must not be a proper noun.
            if let Some(first_char) = word.chars().next() {
                if first_char.is_ascii_uppercase() {
                    return false;
                }
            }

            let word_lower = word.to_lowercase();
            // Criterion 1: Must be 4 letters or longer.
            // Criterion 3: Must contain the center letter.
            // Criterion 4 & 5: Must only contain letters from the grid.
            word_lower.len() >= 4 &&
            word_lower.contains(center_letter) &&
            word_lower.chars().all(|c| valid_letters.contains(&c))
        })
        .collect();

    // Use the `partition` method to split the single vector into pangrams and non-pangram words.
    let (pangrams, non_pangram_words): (Vec<String>, Vec<String>) = all_valid_words.into_iter()
        .partition(|word| {
            let word_chars: HashSet<char> = word.chars().collect();
            word_chars.is_superset(&valid_letters)
        });

    let total_words = non_pangram_words.len() + pangrams.len();
    println!("Found {} total words:", total_words);

    // Combine pangrams and other words for a single list
    let mut all_words = pangrams.clone();
    all_words.extend(non_pangram_words.clone());

    // Determine the max word length for column alignment
    let max_len = all_words.iter()
        .map(|word| word.len())
        .max()
        .unwrap_or(0);

    // Print the words in dynamic columns
    let pangram_bold_start = "\x1B[1m";
    let pangram_bold_end = "\x1B[0m";
    
    // Use the `chunks` iterator to print the words in dynamic columns.
    for chunk in all_words.chunks(num_columns) {
        let mut row_output = String::new();
        for (i, word) in chunk.iter().enumerate() {
            // Apply padding to the word itself, and then wrap in bold codes if it's a pangram.
            let padding_width = if i == chunk.len() - 1 {
                word.len()
            } else {
                max_len + COLUMN_SPACING
            };
            let mut formatted_word = format!("{:<width$}", word, width = padding_width);

            if pangrams.contains(word) {
                formatted_word = format!("{}{}{}", pangram_bold_start, formatted_word, pangram_bold_end);
            }
            
            row_output.push_str(&formatted_word);
        }
        println!("{}", row_output.trim());
    }

    Ok(())
}

