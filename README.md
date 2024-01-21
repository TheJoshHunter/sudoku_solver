# Sudoku Solver

A simple sudoku solver made with SvelteKit and Tauri. Sudoku's are solved using a backtracking algorithm implemented in Rust for speed and memory efficiency.
This tool is considered feature complete, but some changes to improve the UI may be made in the future.

## Usage

Start the program and enter the numbers in the grid. Empty cells can be left blank or filled with a 0. Click the validate button to ensure there are no pre-existing conflicts. Click the solve button to solve the puzzle. The program will display the solution if one exists, or an error message if the puzzle is unsolvable. The puzzle can also be saved to disk as a .json file and loaded the same way. The puzzle can be reset to a blank grid by clicking the reset button.

## Running

Premade binaries for Windows and macOS (Intel) are available in the releases. This application has not been notarized, you will need to allow it in your system settings. This program has not been tested on Linux, but it should work fine.

## Building

1. Clone the repo
2. Install dependencies

```bash
pnpm install
```

3. Install the latest Rust toolchain using the instructions [here](https://www.rust-lang.org/tools/install)
4. Use the tauri cli to build the app

```bash
pnpm run tauri build
```

5. The built app will be in the `src-tauri/target/release` folder

## Contributing

Open a pull request and I'll look at it.

## License

This code is licensed under the MIT license. See the [LICENSE](LICENSE) file for more information.
The solver code has been adapted from TheAlgorithms Rust backtracking example [here](https://github.com/TheAlgorithms/Rust/blob/master/src/backtracking/sudoku.rs). This code is also licensed under the MIT license.
