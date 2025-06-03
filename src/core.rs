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

use std::collections::{HashMap, HashSet};

/// Maximum allowed guesses.
pub const DEFAULT_MAX_GUESSES: usize = 6;

/// First word if one is not provided by the user.
pub const DEFAULT_FIRST_WORD: &str = "reads";

/// Wordle feedback types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feedback {
    Green,
    Yellow,
    Gray,
}

impl Feedback {
    pub fn from_char(char: char) -> Option<Self> {
        match char.to_ascii_lowercase() {
            'g' => Some(Feedback::Green),
            'y' => Some(Feedback::Yellow),
            'b' => Some(Feedback::Gray),
            _ => None,
        }
    }
}

/// Choose the highest‑scoring possible candidate or return None if there are no possible words.
pub fn select_guess(candidates: &[String]) -> Option<(&str, usize)> {
    if candidates.is_empty() {
        return None;
    }

    // Rewards unique letters and vowels (probably not the best, but it seems to do fine)
    let mut best_word_and_score = (&candidates[0][..], 0);
    for word in candidates {
        let unique_letters = word.chars().collect::<HashSet<_>>().len();
        let unique_vowels = word
            .chars()
            .filter(|char| "aeiouy".contains(*char))
            .collect::<HashSet<_>>()
            .len();
        let score = unique_letters + unique_vowels;
        if score > best_word_and_score.1 {
            best_word_and_score = (word.as_str(), score);
        }
    }
    Some((best_word_and_score.0, candidates.len()))
}

/// Apply feedback to update state in‑place.
pub fn apply_feedback(
    guess: &str,
    feedback: &[Feedback; 5],
    green: &mut [Option<char>; 5],
    yellow: &mut Vec<(char, usize)>,
    gray: &mut HashSet<char>,
    min_counts: &mut HashMap<char, usize>,
) {
    let mut round_counts = HashMap::new();

    // Update green, yellow, and gray sets
    for (index, char) in guess.chars().enumerate() {
        match feedback[index] {
            Feedback::Green => {
                green[index] = Some(char);
                *round_counts.entry(char).or_default() += 1;
            }
            Feedback::Yellow => {
                yellow.push((char, index));
                *round_counts.entry(char).or_default() += 1;
            }
            Feedback::Gray => {
                gray.insert(char);
            }
        }
    }

    // Update minimum counts
    for (char, &count) in &round_counts {
        let entry = min_counts.entry(*char).or_default();
        if count > *entry {
            *entry = count;
        }
    }
}

/// Filter candidates against accumulated feedback.
pub fn filter_candidates(
    words: &[String],
    green: &[Option<char>; 5],
    yellow: &[(char, usize)],
    gray: &HashSet<char>,
    min_counts: &HashMap<char, usize>,
) -> Vec<String> {
    words
        .iter()
        .filter(|w| {
            let candidate_chars: Vec<char> = w.chars().collect();

            // Greens
            for (index, required_char) in green.iter().enumerate() {
                if let Some(required) = required_char {
                    if candidate_chars[index] != *required {
                        return false;
                    }
                }
            }

            // Yellows
            for &(yellow_char, position) in yellow {
                if candidate_chars[position] == yellow_char
                    || !candidate_chars.contains(&yellow_char)
                {
                    return false;
                }
            }

            // Grays
            for &gray_char in gray {
                if !min_counts.contains_key(&gray_char) && candidate_chars.contains(&gray_char) {
                    return false;
                }
            }

            // Minimum counts
            for (&char, &minimum_count) in min_counts {
                if candidate_chars
                    .iter()
                    .filter(|&&candidate| candidate == char)
                    .count()
                    < minimum_count
                {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect()
}
