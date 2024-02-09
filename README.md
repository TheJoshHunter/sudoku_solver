# Sudoku Solver

A simple sudoku solver made with SvelteKit and Tauri. Sudoku's are solved using a backtracking algorithm implemented in Rust for speed and memory efficiency.
This tool is considered feature complete, but some changes to improve the UI may be made in the future.

## Usage

Start the program and enter the numbers in the grid. Empty cells can be left blank or filled with a 0. Click the validate button to ensure there are no pre-existing conflicts. Click the solve button to solve the puzzle. The program will display the solution if one exists, or an error message if the puzzle is unsolvable. The puzzle can also be saved to disk as a .sudoku file and loaded the same way. The puzzle can be reset to a blank grid by clicking the reset button.

## Running

Premade binaries for Windows and macOS (Universal) are available in the releases. This application has not been notarized by Apple, you may need to allow the app to run in your system settings.
I have not had the chance to test the Linux build, but it should work. If you can fix any platform specific issues, please open a pull request so I can merge it.

## Building

1. Clone the repo
2. Install dependencies with pnpm or npm

```bash
pnpm install
```

or

```bash
npm install
```

3. Install the latest Rust toolchain for your machine using the instructions [here](https://www.rust-lang.org/tools/install)
4. Use the tauri cli to build the app. Instructions for [building for Windows](https://tauri.app/v1/guides/building/windows), [building for macOS](https://tauri.app/v1/guides/building/macos), and [building for Linux](https://tauri.app/v1/guides/building/linux) from the Tauri documentation can be used for more detailed instructions. In general, the following commands can be used to build the app:

```bash
pnpm tauri build
```

or

```bash
npm run tauri build
```

5. The built app will either be in the `src-tauri/target/release` folder or in a subfolder containing the specified target. Reference the console output for the location of the built app as it depends on the target platform.

## Contributing

Open a pull request and I'll look at it.

## Changelog

Version 0.3.0: Renamed the save file extension to .sudoku, fixed a few odd issues relating to resetting the board, and removed some unnecessary code used for debugging. First release with universal macOS binary.

Version 0.2.0: Added more information to the success message with number of moves and checks, and other small backend improvements.

Version 0.1.0: Initial release

## License

This code is licensed under the MIT license. See the [LICENSE](LICENSE) file for more information.
The solver code has been adapted from TheAlgorithms Rust backtracking example [here](https://github.com/TheAlgorithms/Rust/blob/master/src/backtracking/sudoku.rs). This code is also licensed under the MIT license.
