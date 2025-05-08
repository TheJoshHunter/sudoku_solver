<script lang="ts">
    import Check from "$lib/Check.svelte";
    import {
        Dropdown,
        DropdownItem,
        DropdownMenu,
        DropdownToggle,
    } from "@sveltestrap/sveltestrap";
    import { invoke } from "@tauri-apps/api/core";
    import { downloadDir } from "@tauri-apps/api/path";
    import { open, save } from "@tauri-apps/plugin-dialog";
    import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
    import "../../node_modules/bootstrap/dist/css/bootstrap.css";

    interface SudokuPreset {
        name: string;
        board: number[][];
    }

    interface SudokuResult {
        board: number[][];
        solved: boolean;
        moves: number;
        checks: number;
    }

    let presets: SudokuPreset[] = [
        {
            name: "easy",
            board: [
                [5, 3, 0, 2, 9, 0, 0, 0, 4],
                [0, 0, 2, 7, 4, 3, 5, 0, 0],
                [4, 0, 9, 0, 5, 0, 1, 3, 0],
                [0, 0, 0, 5, 8, 0, 0, 0, 7],
                [0, 8, 0, 0, 2, 4, 9, 0, 0],
                [2, 0, 0, 1, 0, 9, 0, 0, 0],
                [0, 0, 5, 0, 0, 2, 8, 7, 1],
                [0, 9, 0, 0, 0, 7, 0, 0, 0],
                [7, 2, 6, 8, 0, 0, 3, 0, 9],
            ],
        },
        {
            name: "test case",
            board: [
                [3, 0, 6, 5, 0, 8, 4, 0, 0],
                [5, 2, 0, 0, 0, 0, 0, 0, 0],
                [0, 8, 7, 0, 0, 0, 0, 3, 1],
                [0, 0, 3, 0, 1, 0, 0, 8, 0],
                [9, 0, 0, 8, 6, 3, 0, 0, 5],
                [0, 5, 0, 0, 9, 0, 6, 0, 0],
                [1, 3, 0, 0, 0, 0, 2, 5, 0],
                [0, 0, 0, 0, 0, 0, 0, 7, 4],
                [0, 0, 5, 2, 0, 6, 3, 0, 0],
            ],
        },
    ];

    // holds the sudoku board, will be a 2d array of numbers, 0 for empty
    let board: number[][] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let board_is_solvable: boolean = false;
    let problem_text: string = "";
    let success_text: string = "";

    let solving: boolean = false;
    let validating: boolean = false;

    async function solve() {
        // validate that the array is 9x9 as Rust will panic if it is too big
        if (board.length != 9 || board[0].length != 9) {
            problem_text = "The board is not 9x9, this should never happen";
            return;
        }
        // call the tauri api to solve the sudoku
        solving = true;
        invoke("solve", { board })
            .then((response) => {
                const result = response as SudokuResult;
                if (result.solved) {
                    board = result.board;
                    success_text = `Solved in ${result.moves} moves and ${result.checks} checks`;
                } else {
                    problem_text = `The board is not solvable, checked in ${result.moves} moves and ${result.checks} checks`;
                }
            })
            .catch((error) => {
                solving = false;
                // alert the user of the error
                problem_text = error;
            });
        solving = false;
    }

    async function validate() {
        // validate that the array is 9x9 as Rust will panic if it is too big
        if (board.length != 9 || board[0].length != 9) {
            problem_text = "The board is not 9x9, this should never happen";
            return;
        }

        // check that each value is between 0 and 9 (also checks if they fit in a rust u8)
        board.forEach((row) => {
            row.forEach((cell) => {
                if (cell < 0 || cell > 9) {
                    problem_text = "The board contains invalid values";
                    return;
                }
            });
        });

        // call the tauri api to solve the sudoku
        validating = true;
        invoke("validate", { board })
            .then((response) => {
                // response is a boolean of whether the board is valid or not
                if (response) {
                    board_is_solvable = true;
                    problem_text = ""; // make the problem text blank
                } else {
                    problem_text =
                        "The board is not valid and cannot be solved";
                    board_is_solvable = false;
                }
                validating = false;
            })
            .catch((error) => {
                alert(error);
                board_is_solvable = false;
                validating = false;
            });
    }

    async function save_board() {
        const defaultName = "board.sudoku";
        const filePath = await save({
            defaultPath: (await downloadDir()) + "/" + defaultName,
            filters: [
                {
                    name: "Sudoku Board",
                    extensions: ["sudoku"],
                },
            ],
        });
        if (!filePath) {
            return;
        }
        await writeTextFile(filePath, JSON.stringify(board));
    }

    async function load_board() {
        const selected = await open({
            multiple: false,
            filters: [
                {
                    name: "Sudoku Board",
                    extensions: ["sudoku"],
                },
            ],
        });
        if (Array.isArray(selected)) {
            return; // should never happen
        } else if (selected === null) {
            return; // nothing to do
        } else {
            board = JSON.parse(await readTextFile(selected)) as number[][];
        }
    }

    function reset() {
        // Sometimes svelte does not change the entire board, so set each cell to 0
        for (let i = 0; i < 9; i++) {
            for (let j = 0; j < 9; j++) {
                board[i][j] = 0;
            }
        }
        board_is_solvable = false;
        problem_text = "";
        success_text = "";
    }

    // i used gemini to create this
    function calcStyle(i: number, j: number): string {
        // Check for invalid input.
        if (i < 0 || i > 8 || j < 0 || j > 8) {
            return "#FFFFFF";
        }

        // Calculate the 3x3 box index.
        const boxRow = Math.floor(i / 3);
        const boxCol = Math.floor(j / 3);
        const boxIndex = boxRow * 3 + boxCol;

        // Determine the color based on the box index.
        // Alternate between two slightly different gray shades.
        if (boxIndex % 2 === 0) {
            return "grey-background"; // Light gray
        } else {
            return "white-background"; // White
        }
    }
</script>

<svelte:head>
    <title>Sudoku Solver</title>
</svelte:head>

<div class="container">
    <Check />

    <table class="table table-bordered table-sm">
        <tbody>
            {#each board as row, i}
                <tr>
                    {#each row as cell, j}
                        <td class={`${calcStyle(i, j)}`}>
                            <input
                                style="width: 100%; border: none;"
                                class={`no-arr ${calcStyle(i, j)}`}
                                type="number"
                                bind:value={board[i][j]}
                                min="0"
                                max="9"
                                name={`row ${i} col ${j}`}
                            />
                        </td>
                    {/each}
                </tr>
            {/each}
        </tbody>
    </table>

    <br />

    <p>
        Enter the sudoku board above, with 0s for empty cells. Click validate to
        check if the board is valid. If it is valid, click solve to solve it.
        You can also load a preset board from the dropdown.
    </p>

    {#if problem_text}
        <p class="text-danger">
            Something went wrong: {problem_text}
        </p>
    {/if}

    {#if board_is_solvable}
        <p class="text-success">
            The board is (theoretically) solvable! Click solve to solve it.
        </p>
    {/if}

    {#if success_text}
        <p class="text-success">
            {success_text}
        </p>
    {/if}

    <Dropdown class="d-inline-block" group>
        <DropdownToggle caret>Presets</DropdownToggle>
        <DropdownMenu>
            {#each presets as preset}
                <DropdownItem
                    on:click={() => {
                        board = preset.board;
                    }}
                >
                    {preset.name}
                </DropdownItem>
            {/each}
        </DropdownMenu>
    </Dropdown>

    <button class="btn btn-primary" disabled={validating} on:click={validate}>
        {#if validating}
            <span
                class="spinner-border spinner-border-sm"
                role="status"
                aria-hidden="true"
            ></span>
            Validating...
        {:else}
            Validate Board
        {/if}
    </button>

    <button
        class="btn btn-primary"
        disabled={solving || !board_is_solvable}
        on:click={solve}
    >
        {#if solving}
            <span
                class="spinner-border spinner-border-sm"
                role="status"
                aria-hidden="true"
            ></span>
            Solving...
        {:else}
            Solve
        {/if}
    </button>
    <button class="btn btn-danger" on:click={reset}> Reset </button>
    <button class="btn btn-secondary" on:click={save_board}>
        Save Board
    </button>
    <button class="btn btn-secondary" on:click={load_board}>
        Load Board
    </button>
</div>

<style>
    .grey-background {
        background-color: #d3d3d3;
    }
    .light-background {
        background-color: #ffffff;
    }

    /* Hide Arrows */
    /* Chrome, Safari, Edge, Opera */
    input.no-arr::-webkit-outer-spin-button,
    input.no-arr::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }

    /* Firefox */
    input[type="number"].no-arr {
        -moz-appearance: textfield;
    }
</style>
