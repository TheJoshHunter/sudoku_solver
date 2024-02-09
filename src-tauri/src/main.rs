// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// this should be set to false for production
// should automatically be set to false when building for release
// This is bad and should not be used in a serious application, don't copy this
static DEBUG: bool = if cfg!(debug_assertions) { true } else { false };

mod sudoku;
pub use sudoku::Sudoku; // load in the sudoku implementation

// exists to easily check if we are running in tauri or not
#[tauri::command]
fn check() -> bool {
    true // just to make sure it works
}

/**
 * This function checks to see if a given sudoku board is valid.
 * This function has a quick escape as once an issue is found, the function returns false to save time.
 * @param board The sudoku board to check.
 * @return True if the board is valid, false otherwise.
 */
#[tauri::command]
fn validate(board: [[u8; 9]; 9]) -> bool {
    let mut something_other_than_zero = false;

    // check rows
    for row in 0..9 {
        // create a set to store the numbers we have seen
        let mut row_set = std::collections::HashSet::new();
        // iterate over the columns
        for col in 0..9 {
            if board[row][col] != 0 {
                something_other_than_zero = true; // only need this once

                if row_set.contains(&board[row][col]) {
                    // we have seen this number before in this row, so the board is invalid
                    if DEBUG {
                        println!("Row {} is invalid", row);
                    }
                    return false;
                } else {
                    // new number, add it to the set
                    row_set.insert(board[row][col]);
                }
            }
        }
    }

    // check columns
    for col in 0..9 {
        let mut col_set = std::collections::HashSet::new();
        for row in 0..9 {
            if board[row][col] != 0 {
                if col_set.contains(&board[row][col]) {
                    if DEBUG {
                        println!("Column {} is invalid", col);
                    }
                    return false;
                } else {
                    col_set.insert(board[row][col]);
                }
            }
        }
    }

    // check 3x3 squares
    for row in 0..3 {
        for col in 0..3 {
            let mut square_set = std::collections::HashSet::new();
            for i in 0..3 {
                for j in 0..3 {
                    if board[row * 3 + i][col * 3 + j] != 0 {
                        if square_set.contains(&board[row * 3 + i][col * 3 + j]) {
                            if DEBUG {
                                println!("Square {} {} is invalid", row, col);
                            }
                            return false;
                        } else {
                            square_set.insert(board[row * 3 + i][col * 3 + j]);
                        }
                    }
                }
            }
        }
    }

    if !something_other_than_zero {
        if DEBUG {
            println!("Board is invalid as it is all 0s");
        }
        return false; // for the sake of performance, if the board is all 0s, lets not try to solve it
    } else {
        if DEBUG {
            println!("Board is valid");
        }
        return true; // if we get here, the board is valid
    }
}

/**
* JS accessible function to solve a sudoku board.
* Uses the inner solve function to handle the actual solving.
* @param board The sudoku board to solve.
* @return The solved sudoku board.
*/
#[tauri::command]
fn solve(board: [[u8; 9]; 9]) -> Sudoku {
    // print the initial board
    let mut solver = Sudoku::new(board);
    if DEBUG {
        println!("Solving board: ");
        solver.print_board();
    }

    // this looks funny, but our solve function returns whether or not the board was solved not the solved board
    if solver.solve() {
        println!("Board solved!");
        return solver;
    } else {
        println!("Board could not be solved!");
        return solver;
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check, validate, solve]) // insert the functions to be called from JS
        .run(tauri::generate_context!())
        .expect("Something went wrong while running Tauri application.");
}
