// This code borrowed from https://github.com/TheAlgorithms/Rust/blob/master/src/backtracking/sudoku.rs
// Original code licensed under MIT License

pub struct Sudoku {
    board: [[u8; 9]; 9],
}

impl Sudoku {
    pub fn new(board: [[u8; 9]; 9]) -> Self {
        Self { board }
    }

    fn find_empty_cell(&self) -> Option<(usize, usize)> {
        // Find a empty cell in the board (returns None if all cells are filled)
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    return Some((i, j));
                }
            }
        }

        None
    }

    fn check(&self, index_tuple: (usize, usize), value: u8) -> bool {
        // resolve he x and y position of the cell
        let (y, x) = index_tuple;

        // checks if the value to be added in the board is an acceptable value for the cell

        // checking through the row
        for i in 0..9 {
            if self.board[i][x] == value {
                return false;
            }
        }
        // checking through the column
        for i in 0..9 {
            if self.board[y][i] == value {
                return false;
            }
        }

        // checking through the 3x3 block of the cell
        let sec_row = y / 3;
        let sec_col = x / 3;

        for i in (sec_row * 3)..(sec_row * 3 + 3) {
            for j in (sec_col * 3)..(sec_col * 3 + 3) {
                if self.board[i][j] == value {
                    return false;
                }
            }
        }

        true
    }

    pub fn solve(&mut self) -> bool {
        // grabs the first empty cell and tries to solve the board
        let empty_cell = self.find_empty_cell();

        // if there is an empty cell
        if let Some((y, x)) = empty_cell {
            // going through all possible values for the cell
            for val in 1..10 {
                // if the new value is acceptable
                if self.check((y, x), val) {
                    // setting the value
                    self.board[y][x] = val;
                    // trying to solve the board using the new value
                    // call the function again to set the next empty cell (which should be the next empty cell after the current one)
                    if self.solve() {
                        return true;
                    }
                    // backtracking if the board cannot be solved using current configuration
                    self.board[y][x] = 0
                }
            }
        } else {
            // if the board is complete (no empty cells)
            return true;
        }

        // returning false the board cannot be solved using current configuration
        false
    }

    pub fn print_board(&self) {
        // helper function to display board

        let print_3_by_1 = |arr: Vec<u8>, last: bool| {
            let str = arr
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            if last {
                println!("{str}",);
            } else {
                print!("{str} | ",);
            }
        };

        for i in 0..9 {
            if i % 3 == 0 && i != 0 {
                println!("- - - - - - - - - - - - - -")
            }

            print_3_by_1(self.board[i][0..3].to_vec(), false);
            print_3_by_1(self.board[i][3..6].to_vec(), false);
            print_3_by_1(self.board[i][6..9].to_vec(), true);
        }
    }
}
