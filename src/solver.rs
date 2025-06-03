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

use crate::core::*;
use prompted::input;
use std::collections::{HashMap, HashSet};

/// Solves the wordle.
pub fn solve(
    word_list: Vec<String>,
    test_answer: Option<String>,
    initial_state: Option<String>,
    chosen_first: Option<String>,
    max_guesses: usize,
) {
    println!(
        "Wordle SolvRS - {}{}",
        if test_answer.is_some() {
            "(Test Mode) Answer: '"
        } else {
            "(Manual Mode)"
        },
        test_answer
            .as_deref()
            .map_or("".to_string(), |word| format!("{}'", word))
    );

    let mut green: [Option<char>; 5] = [None; 5];
    let mut yellow: Vec<(char, usize)> = Vec::new();
    let mut gray: HashSet<char> = HashSet::new();
    let mut min_counts: HashMap<char, usize> = HashMap::new();
    let mut remaining_rounds = max_guesses;
    let first_word = chosen_first.unwrap_or_else(|| DEFAULT_FIRST_WORD.to_string());

    // Load previous state in finish mode
    if test_answer.is_none() {
        if let Some(state) = initial_state {
            load_state(
                &state,
                &mut green,
                &mut yellow,
                &mut gray,
                &mut min_counts,
                &mut remaining_rounds,
            );
        }
    }

    // Main loop
    for round in 1..=remaining_rounds {
        // Find valid words
        let candidates = filter_candidates(&word_list, &green, &yellow, &gray, &min_counts);

        // Select guess
        let guess_count = round + max_guesses - remaining_rounds;
        let (guess, total_candidates) = if guess_count == 1 {
            (first_word.as_str(), word_list.len())
        } else {
            select_guess(&candidates).unwrap_or_else(|| {
                eprintln!("Error: no possible candidates, exiting.");
                std::process::exit(1)
            })
        };

        println!(
            "Guess {}: {} ({} candidates)",
            guess_count, guess, total_candidates
        );

        // Accept feedback
        let feedback = match test_answer.as_deref() {
            Some(word) => generate_feedback(guess, word),
            None => manual_feedback(guess),
        };

        // Check for success
        if feedback.iter().all(|&f| f == Feedback::Green) {
            println!("Solved in {} rounds.", guess_count);
            return;
        }

        // Record feedback
        apply_feedback(
            guess,
            &feedback,
            &mut green,
            &mut yellow,
            &mut gray,
            &mut min_counts,
        );
    }

    println!("Failed to solve the puzzle in {} guesses.", max_guesses);
}

/// Prompt the user and parse a 5‑char feedback string.
fn manual_feedback(guess: &str) -> [Feedback; 5] {
    loop {
        // Prompt for feedback
        let feedback = input!(
            "Enter feedback for `{}` (g=green, y=yellow, b=gray): ",
            guess
        )
        .trim()
        .to_string();

        // Check for empty input
        if feedback.is_empty() {
            eprintln!("Skipped feedback. Exiting.");
            std::process::exit(0);
        }

        // Check length
        if feedback.len() != 5 {
            eprintln!("Error: feedback must be 5 characters.");
            continue;
        }

        // Check for valid characters
        let mut feedback_array = [Feedback::Gray; 5];
        for (index, char) in feedback.chars().enumerate() {
            feedback_array[index] = Feedback::from_char(char).unwrap_or_else(|| {
                eprintln!("Error: invalid feedback `{}`", char);
                std::process::exit(1)
            });
        }

        return feedback_array;
    }
}

/// Automated feedback generator.
fn generate_feedback(guess: &str, answer: &str) -> [Feedback; 5] {
    let mut result = [Feedback::Gray; 5];
    let mut remaining_chars: Vec<Option<char>> = answer.chars().map(Some).collect();

    // First pass: mark green
    for (index, (guess_char, answer_char)) in guess.chars().zip(answer.chars()).enumerate() {
        if guess_char == answer_char {
            result[index] = Feedback::Green;
            remaining_chars[index] = None;
        }
    }

    // Second pass: mark yellow
    for (index, guess_char) in guess.chars().enumerate() {
        if result[index] == Feedback::Gray {
            if let Some(pos) = remaining_chars.iter().position(|&c| c == Some(guess_char)) {
                result[index] = Feedback::Yellow;
                remaining_chars[pos] = None;
            }
        }
    }
    result
}

/// Loads a comma‑separated history of guess+feedback pairs.
fn load_state(
    data: &str,
    green: &mut [Option<char>; 5],
    yellow: &mut Vec<(char, usize)>,
    gray: &mut HashSet<char>,
    min_counts: &mut HashMap<char, usize>,
    remaining: &mut usize,
) {
    let mut applied_guesses = 0;

    for entry in data.split(',').map(str::trim) {
        if entry.len() != 10 {
            eprintln!("Warning: skipping invalid entry `{}`", entry);
            continue;
        }
        let (guess, feedback_entry) = entry.split_at(5);
        let mut feedback = [Feedback::Gray; 5];
        let mut valid = true;

        for (index, char) in feedback_entry.chars().enumerate() {
            match Feedback::from_char(char) {
                Some(feedback_char) => feedback[index] = feedback_char,
                None => {
                    eprintln!("Warning: invalid feedback `{}` in `{}`", char, entry);
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            continue;
        }
        apply_feedback(guess, &feedback, green, yellow, gray, min_counts);
        applied_guesses += 1;
    }
    *remaining = remaining.saturating_sub(applied_guesses);
}
