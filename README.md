# Nonograms

## Usage

Download the repository and open a terminal inside the nonogram folder. Enter the command `cargo run <path>`, where `<path>` is the path to a `.nono` file containing the contents of a nonogram puzzle. For example, try `cargo run puzzles/test.nono`.

It is recommended that you adjust your terminal display settings to a reduced font size and adjusting cell width to around `1.30` such that the entire grid fits in your terminal, and each cell is roughly square shaped.

Once the program is run the following keys can be used to solve the puzzle:

* Arrow Keys: Move the cursor
* Z: Fill a square
* X: Cross a square
* Q: Quit the program

Resizing the terminal to a size smaller than the board will result in the program crashing!


## Todo

* Choose puzzle file from terminal
* Highlight row and column where the cursor is at
* Implement mouse support
* Cross out completed constraints
* Check for finished puzzle
* Prevent program from crashing upon resize
* Allow program to download puzzles from the Internet