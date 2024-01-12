// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const DEBUG: bool = true; // set to true to enable debug printing

// exists to easily check if we are running in tauri or not
#[tauri::command]
fn check() -> bool {
    true // just to make sure it works
}

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
    if DEBUG {
        println!("Validate called on board: ");
        print_board(&board);
    }

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
 * This function checks to see if a given sudoku board is solved.
 * Modified version of the validate function that fails if it finds a 0.
 * @param board The sudoku board to check.
 * @return True if the board is solved, false otherwise.
 */
fn is_solved(board: &[[i32; 9]; 9]) -> bool {
    if DEBUG {
        println!("Checking if board is solved");
    }

    // check rows
    for row in 0..9 {
        // create a set to store the numbers we have seen
        let mut row_set = std::collections::HashSet::new();
        // iterate over the columns
        for col in 0..9 {
            if board[row][col] != 0 {
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
            } else {
                // if we find a 0, the board is not solved
                if DEBUG {
                    println!("Row {} contains a 0", row);
                }
                return false;
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
            } else {
                if DEBUG {
                    println!("Column {} contains a 0", col);
                }
                return false;
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
                    } else {
                        return false;
                    }
                }
            }
        }
    }
    if DEBUG {
        println!("Board is solved!");
    }
    return true;
}

/**
 * This function returns the number of empty spaces in a given sudoku board.
 * @param board The sudoku board to check.
 * @return The number of empty spaces in the board as i32
 */
fn empty_spaces(board: &[[i32; 9]; 9]) -> i32 {
    let mut count = 0;
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == 0 {
                count += 1;
            }
        }
    }
    return count;
}

/**
* JS accessible function to solve a sudoku board.
* Uses the inner solve function to handle the actual solving.
* @param board The sudoku board to solve.
* @return The solved sudoku board.
*/
#[tauri::command]
fn solve(mut board: [[i32; 9]; 9]) -> Result<[[i32; 9]; 9], ()> {
    // print the initial board
    println!("Solving board: ");
    print_board(&board);

    // run the inner solve function
    return solve_inner(&mut board, (0, 0));
}

/**
* Inner solve function to handle the actual solving of the board.
* This function uses hill climbing to solve the board.
* @param board The sudoku board to solve.
* @param start_space The space to start solving at (used for backtracking, starts at 0,0 but will be updated if 0,0 is not empty).
* @return The solved sudoku board.
*/
fn solve_inner(
    board: &mut [[i32; 9]; 9],
    mut start_space: (usize, usize),
) -> Result<[[i32; 9]; 9], ()> {
    let mut empty_spaces = Vec::new();

    // find all the empty spaces to fill (we don't want to change existing numbers)
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == 0 {
                // if the space is empty, add it to the vector
                empty_spaces.push((row, col));
            }
        }
    }

    // if the start space is not empty, find the next empty space
    if board[start_space.0][start_space.1] != 0 {
        for i in 0..empty_spaces.len() {
            if empty_spaces[i] == start_space {
                start_space = empty_spaces[i + 1];
                break;
            }
        }
    }

    // iterate over the empty spaces
    for i in 0..empty_spaces.len() {
        // start from out start space
        let (row, col) = start_space;

        // iterate over the numbers 1-9
        for num in 1..10 {
            // set the current empty space to the current number
            board[row][col] = num;

            // check if the board is valid
            if validate(*board) {
                // if the board is valid, check if it is solved
                if is_solved(&board) {
                    // if the board is solved, return it
                    return Ok(*board);
                } else {
                    // if the board is not solved, solve the next empty space
                    let solved_board = solve_inner(board, empty_spaces[i + 1]).unwrap();
                    if is_solved(&solved_board) {
                        return Ok(solved_board);
                    }
                }
            }
        }

        // if we get here, we have tried all numbers and none of them worked
        // set the current empty space back to 0
        board[row][col] = 0;
        // return the board
        return Err(());
    }

    // if we get here, we have tried all empty spaces and none of them worked
    // return the board
    return Ok(*board);
}

/*
Function to print the board to the console.
The board is sent as a 2D array of i32s.
The board is borrowed as it is not modified.
*/
fn print_board(board: &[[i32; 9]; 9]) {
    println!("N | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |");
    println!("---------------------------------------");
    for row in 0..9 {
        print!("{} | ", row);
        for col in 0..9 {
            print!("{} | ", board[row][col]);
        }
        println!();
        println!("---------------------------------------");
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check, greet, validate, solve]) // insert the functions to be called from JS
        .run(tauri::generate_context!())
        .expect("Something went wrong while running Tauri application.");
}
