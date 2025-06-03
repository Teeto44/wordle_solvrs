# Wordle SolvRS

## _Feature complete CLI wordle bot in Rust_

### Warning: This project is not actively maintained, please open an issue if any of the functionality is outright dysfunctional but other than that I have no intention of updating it.

## Usage

### Basic Commands

- `-h` or `--help`: Show the help message.
- `-t` or `--test <word>`: Loads the solver in test mode.
- `-f` or `--first <word>`: Make solver use a specified first word.
- `-s` or `--state <state>`: Load a given game state.
- `-w` or `--words <path>`: Use a custom word list.
- `-g` or `--guesses <number>`: Set custom maximum guesses.

### State Loading

After `-s`, provide a string with the current state of the game.  
The guesses should be separated by commas and formatted as a tuple of guess and feedback.
This will be ignored in test mode.
**Example**: `slateybbbb,pastsgbbbg`

### Feedback Format

- `g`: Green (correct letter in the correct position).
- `y`: Yellow (correct letter in the wrong position).
- `b`: Gray (letter not in the word).  
  **Example**: `gbybb`

### Test Mode

When you launch the solver in test mode and a provided word, it will do its best to solve the puzzle using a given word
as the answer.

### Additional Notes

- No arguments are required, everything has default values.
- The default word list is embedded in the binary, so the executable is fully portable.
- The default first word is "reads", it is hardcoded because with the entire word list available
  it will always pick adieu, and I just don't like it.

## License

[MIT](https://choosealicense.com/licenses/mit/)

