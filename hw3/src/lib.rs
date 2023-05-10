//! Chomp AI
//! Bradley Thompson and Bart Massey 2023

use std::fmt::Display;

/// Maximum number of rows the AI can handle.
const MAX_ROWS: usize = 4;
/// Maximum number of columns the AI can handle.
const MAX_COLS: usize = 5;

/// A Chomp board.
#[derive(Debug, Clone)]
pub struct Chomp {
    /// The number of rows for this game.
    pub nrows: usize,

    /// The number of columns for this game.
    pub ncols: usize,

    /// The board. `true` is an un-eaten square, `false` is
    /// an eaten square.
    pub board: [[bool; MAX_COLS]; MAX_ROWS],
}

impl Chomp {
    /// Make a new Chomp board with the given size for this game.
    ///
    /// # Panics
    /// Panics if the requested board size is larger than the AI
    /// can handle, or has zero rows or columns.
    pub fn new(nrows: usize, ncols: usize) -> Self {
        assert!(nrows > 0, "not enough rows to play");
        assert!(ncols > 0, "not enough columns to play");
        assert!(
            nrows <= MAX_ROWS,
            "too many rows ({} > {}) for AI",
            nrows,
            MAX_ROWS
        );
        assert!(
            ncols <= MAX_COLS,
            "too many columns ({} > {}) for AI",
            ncols,
            MAX_COLS
        );
        let mut chomp = Self {
            nrows,
            ncols,
            board: [[false; MAX_COLS]; MAX_ROWS],
        };
        for i in 0..nrows {
            for j in 0..ncols {
                chomp.board[i][j] = true;
            }
        }
        chomp
    }

    /// Make a move on the current board, "eating" all cells
    /// below `row` and to the right of `col` inclusive.
    pub fn make_move(&mut self, row: usize, col: usize) {
        for i in row..self.nrows {
            for j in col..self.ncols {
                self.board[i][j] = false;
            }
        }
    }

    /// Returns `Some` winning move for this position as `(row, col)`.
    /// Returns `None` if there is no winning move in this position.
    ///
    /// # Strategy
    ///
    /// ```text
    /// winning-move(posn):
    ///     for each remaining row r
    ///         for each remaining column c in r
    ///             if r = 0 and c = 0
    ///                 continue
    ///             p ← copy of posn
    ///             chomp r, c from p
    ///             m ← winning-move(p)
    ///             if no winning move is returned
    ///                 return the move r, c
    ///    return no winning move
    /// ```
    pub fn winning_move(&self) -> Option<(usize, usize)> {
        /// Nested recursive function which explores all moves until it finds a row/col
        /// pair that cannot be followed with a non-losing moving.
        fn winning_move(chmp: &mut Chomp) -> Option<(usize, usize)> {
            for (r, row) in chmp.board.iter().enumerate() {
                let mut row_iter = row.iter();
                if row_iter.any(|x| *x) {
                    for (c, col) in row_iter.enumerate() {
                        // If this col is already false, just skip, invalid move.
                        // If move is (0, 0), always losing, skip that too.
                        if !*col || (r == 0 && c == 0) {
                            continue;
                        }
                        let mut p = chmp.clone();
                        p.make_move(r, c);
                        let m = winning_move(&mut p);
                        if m.is_none() {
                            return Some((r, c));
                        }
                    }
                }
            }
            None
        }
        let mut mutable_self = self.clone();
        winning_move(&mut mutable_self)
    }
}

impl Display for Chomp {
    /// Convert a Chomp board into a 2D grid of chars, in a String, with rows separated by newline.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board_str = self
            .board
            .iter()
            .map(|row| -> String {
                // Build the row and don't intersperse chars with any separator so that they form a solid row.
                row.iter()
                    .map(|sq| if *sq { "#" } else { "." })
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n"); // Intersperse rows with newline so the board is 2D.
        write!(f, "{}", board_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let c = Chomp::new(2, 2);
        for i in 0..MAX_ROWS {
            for j in 0..MAX_COLS {
                let square = c.board[i][j];
                if i >= 2 || j >= 2 {
                    assert!(!square);
                } else {
                    assert!(square);
                }
            }
        }
    }

    #[test]
    fn test_make_move() {
        let mut c = Chomp::new(2, 3);
        c.make_move(1, 1);
        assert!(c.board[0][0]);
        assert!(!c.board[1][1]);
        assert!(!c.board[1][2]);
    }

    #[test]
    fn test_winning_move() {
        let mut c = Chomp::new(2, 2);
        assert!(c.winning_move().is_some());
        c.make_move(1, 1);
        assert!(c.winning_move().is_none());
    }
}
