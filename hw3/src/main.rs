//! Command-line Chomp player  
//! Bradley Thompson and Bart Massey 2023
//!
//! This player repeatedly
//! * Displays the board
//! * Prompts the human for a move until a legal move is obtained
//! * Makes the human move on the board
//! * Displays the board
//! * Gets a winning computer move from the AI
//!   * If the AI has no winning move, chooses a computer move
//!     by going to the last available row and eating the last
//!     available square in that row
//! * Makes the computer move on the board
//! * Displays the computer move
//! This continues until the game is over,
//! at which point either "you lose" or "you win"
//! is printed depending on the outcome.

use bradleys_random_rust_helpers::parse_num;
use chomp_ai::*;
use prompted::input;

/// Get a move from the human player. The human should
/// supply the move as a row and column (starting from 0)
/// separated by a space, like this.
///
///    2 3
///
/// If the human makes a "bad" move (badly formatted or
/// illegal), this function returns `None`. Otherwise it
/// returns `Some` row and column coordinates of the human
/// move.
fn user_move(posn: &Chomp) -> Option<(usize, usize)> {
    let pair = input!("Enter your row/col pair, separated by a space: ");
    let indices = pair.split(' ').collect::<Vec<&str>>();
    if indices.len() != 2 {
        return None;
    }
    let row: Option<usize> = indices[0].parse().ok();
    let col: Option<usize> = indices[1].parse().ok();

    match (row, col) {
        (Some(row), Some(col)) => {
            // If the play is outside of the game board, or the square is already taken, the move is invalid.
            if row >= posn.nrows || col >= posn.ncols || !posn.board[row][col] {
                None
            } else {
                Some((row, col))
            }
        }
        _ => None,
    }
}

/// Play a game, as described above.
///
/// The program should take two command-line arguments
/// representing the board size: a number of rows and a
/// number of columns for the board. The program should fail
/// (somehow) if the requested board size is too large or
/// negative or not numbers etc.
///
/// Thus, a typical run of the program on a 3×4 board might look like
/// ```text
/// cargo run 3 4
/// ```
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let parsed: Vec<usize> = args.iter().map(|x| parse_num(x)).collect();
    let mut board = Chomp::new(parsed[0], parsed[1]);

    println!(
        "Welcome to Chomp V AI!\nAI: \"Welcome to the game. Do you think you can beat me?\"\n\nStarting board ({} {})...\n\n{}\n",
        parsed[0], parsed[1], board
    );

    loop {
        if let Some((row, col)) = user_move(&board) {
            println!("\nNew move detected! ({}, {})", row, col);
            board.make_move(row, col);
            println!("\nAI: \"Nice move!\"\n");
        } else {
            println!("\nBad input!\n\nAI: \"How embarassing...\"\n")
        }
        println!("Current board state:\n{}\n", board);
    }
}
