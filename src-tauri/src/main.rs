// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/**
 * This function checks to see if a given sudoku board is valid.
 * This function has a quick escape as once an issue is found, the function returns false to save time.
 * @param board The sudoku board to check.
 * @return True if the board is valid, false otherwise.
 */
#[tauri::command]
fn validate(board: [[i32; 9]; 9]) -> bool {
    // check rows
    for row in 0..9 {
        // create a set to store the numbers we have seen
        let mut row_set = std::collections::HashSet::new();
        // iterate over the columns
        for col in 0..9 {
            if board[row][col] != 0 {
                // skip 0s as they are empty spaces
                if row_set.contains(&board[row][col]) {
                    // we have seen this number before in this row, so the board is invalid
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
                            return false;
                        } else {
                            square_set.insert(board[row * 3 + i][col * 3 + j]);
                        }
                    }
                }
            }
        }
    }

    return true;
}

/**
* This function is the solver for the sudoku board.
* To be implemented.
* @param board The sudoku board to solve.
* @return The solved sudoku board.
*/
#[tauri::command]
fn solve(board: [[i32; 9]; 9]) -> [[i32; 9]; 9] {
    // return an empty board for now
    return [[0; 9]; 9];
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, validate, solve])
        .run(tauri::generate_context!())
        .expect("Something went wrong while running Tauri application.");
}
