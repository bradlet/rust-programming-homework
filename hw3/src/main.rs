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

use bradleys_random_rust_helpers::{horizontal_sep, parse_num};
use chomp_ai::*;
use prompted::input;
use text_colorizer::Color;

const H_LINE_LEN: u8 = 64;

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

/// Search the board for the last uneaten square of the last available row, and return that position.
fn computer_move(posn: &Chomp) -> (usize, usize) {
    for (r, row) in posn.board.iter().enumerate().rev() {
        // If there are any active / "uneaten" squares, this is an available row.
        let mut row_iter = row.iter();
        if row_iter.any(|x| *x) {
            for (c, col) in row_iter.enumerate().rev() {
                if *col {
                    return (r, c); // Short-circuit out, we don't need to search anymore.
                }
            }
        }
    }
    (0, 0) // Only available move is the losing move.
}

/// Print out the current board state with surrounding formatting for readability.
fn display_current_board_state(posn: &Chomp) {
    let board_display_sep = horizontal_sep(H_LINE_LEN, None);
    println!(
        "{}\nCurrent board state:\n{}\n{}\n",
        board_display_sep, posn, board_display_sep
    );
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

    println!("Welcome to Chomp V AI!");
    horizontal_sep(H_LINE_LEN, Some(Color::BrightGreen));
    println!(
        "AI: \"Welcome to the game. Do you think you can beat me?\"\n\nStarting board ({}, {})...\n",
        parsed[0], parsed[1]
    );
    display_current_board_state(&board);

    loop {
        // Retrieve move from standard input and enact it on the game board.
        if let Some((row, col)) = user_move(&board) {
            println!("\nNew move detected! ({}, {})", row, col);
            board.make_move(row, col);
        } else {
            println!("\nBad input!\n\nAI: \"How embarassing...\"\n");
            continue;
        }

        display_current_board_state(&board);

        // Check if the user has lost as of this move.
        if !board.board[0][0] {
            horizontal_sep(H_LINE_LEN, Some(Color::BrightRed));
            println!("Game over! You lose!\n\nAI: \"I didn't even have to try...\"\n\n");
            horizontal_sep(H_LINE_LEN, Some(Color::BrightRed));
            break;
        }

        // Check if the AI can make a winning move this turn. If not, return the best "computer move".
        let winning_mv = board.winning_move();
        let (row, col) = if let Some((row, col)) = winning_mv {
            // The AI has detected a winning move, where the next move must lose.
            print!("AI: \"Nice try, but I cannot be beat... ");
            (row, col)
        } else {
            let ai_mv = computer_move(&board);
            // Check if AI lost, or if it's stalling further until it can make the winning move.
            if (0, 0) == ai_mv {
                print!("AI: \"What!?! But that's impossible! Gah... ");
            } else {
                println!("AI: \"Interesting... Maybe your intellect can match my own? ");
            }
            ai_mv
        };

        // Enact the move calculated above and report change in state to user
        println!("My move is ({}, {})\"\n", row, col);
        board.make_move(row, col);
        display_current_board_state(&board);

        // Check if the AI has lost as of this move.
        if !board.board[0][0] {
            horizontal_sep(H_LINE_LEN, Some(Color::BrightGreen));
            println!("Game over! You win!\n\nAI: \"I let you win...\"\n\n");
            horizontal_sep(H_LINE_LEN, Some(Color::BrightGreen));
            break;
        }
    }
}
