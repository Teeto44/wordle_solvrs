/*******************************************************************************
* Wordle SolvRS - A wordle solver written in Rust
*
* The MIT License (MIT)
* Copyright (c) 2025 Teeto44
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to
* deal in the Software without restriction, including without limitation the
* rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
* sell copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
* THE SOFTWARE.
*******************************************************************************/

mod core;
mod solver;

use crate::core::DEFAULT_MAX_GUESSES;
use core::DEFAULT_FIRST_WORD;
use prompted::input;
use solver::solve;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Help message
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage();
        return;
    }

    let word_list = load_words(get_option(&args, &["-w", "--words"]));
    let first_word = get_first_word(&args, &word_list);
    let state = get_option(&args, &["-s", "--state"]);

    // Check for requested test word
    let test_word = match get_option(&args, &["-t", "--test"]) {
        Some(word) if word.len() == 5 && word_list.contains(&word) => Some(word),
        Some(word) => {
            eprintln!("Invalid test word `{}`; ignoring `-t|--test`.", word);
            None
        }
        None => None,
    };

    // Check for max guesses
    let max_guesses = match get_option(&args, &["-g", "--guesses"]) {
        Some(guesses) => match guesses.parse::<usize>() {
            Ok(number) if number > 0 => number,
            _ => {
                eprintln!(
                    "Invalid value for `-g|--guesses`: `{}`; using default `{}`.",
                    guesses, DEFAULT_MAX_GUESSES
                );
                DEFAULT_MAX_GUESSES
            }
        },
        None => DEFAULT_MAX_GUESSES,
    };

    if let Some(answer) = test_word {
        solve(word_list, Some(answer), None, Some(first_word), max_guesses);
    } else {
        solve(word_list, None, state, Some(first_word), max_guesses);
    }
}

/// Helper function to get an option from the command line arguments.
fn get_option(args: &[String], flags: &[&str]) -> Option<String> {
    args.windows(2)
        .find(|w| flags.contains(&w[0].as_str()))
        .map(|w| w[1].clone())
}

/// Determines the first word to use based on command line arguments or defaults.
fn get_first_word(args: &[String], words: &[String]) -> String {
    if let Some(custom) = get_option(args, &["-f", "--first"]) {
        if custom.len() == 5 && words.contains(&custom) {
            return custom;
        }
        eprintln!(
            "Invalid first word `{}`; using default `{}`.",
            custom, DEFAULT_FIRST_WORD
        );
    }
    DEFAULT_FIRST_WORD.to_string()
}

/// Loads a list of words from a file or use a default list if the file is not found.
fn load_words(path: Option<String>) -> Vec<String> {
    let reader = |content: String| {
        content
            .lines()
            .filter(|l| l.len() == 5)
            .map(str::to_string)
            .collect()
    };

    if let Some(path) = path {
        match fs::read_to_string(&path) {
            Ok(txt) => return reader(txt),
            Err(err) => eprintln!(
                "Error: couldn't read {} {}. Using default word list.",
                path, err
            ),
        }
    }

    reader(include_str!("words.txt").to_string())
}

/// Prints basic usage instructions for the program.
fn print_usage() {
    println!("Wordle SolvRS - usage");
    println!("  -h or --help: Show this help message");
    println!("  -t or --test <word>: Loads the solver in test mode");
    println!("  -f or --first <word>: Will make the solver use a specified first word");
    println!("  -s or --state <state>: Will load a given game state");
    println!("  -w or --words <path>: Will make the solver use a custom word list");
    println!("  -g or --guesses <number>: Set custom maximum guesses");
    println!();
    println!("Wordle SolvRS - test mode help");
    println!("  The test mode will make the solver use a given word as the answer");
    println!();
    println!("Wordle SolvRS - state loading help");
    println!("  after -s you can provide a string with the current state of the game");
    println!("  the guesses should be separated by commas, and be a tuple of guess and feedback");
    println!("  this won't work in test mode");
    println!("  Example: slateybbbb,pastsgbbbg");
    println!();
    println!("Wordle SolvRS - feedback help");
    println!("  g: Green (correct letter in correct position)");
    println!("  y: Yellow (correct letter in wrong position)");
    println!("  b: Gray (letter not in word)");
    println!("  Example: gbybb");
    input!("Press enter to exit...");
}
