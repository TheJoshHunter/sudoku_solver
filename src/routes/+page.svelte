<script lang="ts">
    import Check from "$lib/Check.svelte";
    import {
        Dropdown,
        DropdownItem,
        DropdownMenu,
        DropdownToggle,
    } from "@sveltestrap/sveltestrap";
    import { open, save } from "@tauri-apps/api/dialog";
    import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
    import { downloadDir } from "@tauri-apps/api/path";
    import { invoke } from "@tauri-apps/api/tauri";
    import "../../node_modules/bootstrap/dist/css/bootstrap.css";
    // onMount(() => {
    //     // wait till were in the browser to import bootstrap
    //     if (browser) {
    //         import(
    //             "../../node_modules/bootstrap/dist/js/bootstrap.js" as string
    //         );
    //         import(
    //             "../../node_modules/bootstrap/js/dist/dropdown.js" as string
    //         );
    //         import(
    //             "../../node_modules/@popperjs/core/dist/esm/popper.js" as string
    //         );
    //     }
    // });

    interface SudokuPreset {
        name: string;
        board: number[][];
    }

    let presets: SudokuPreset[] = [
        {
            name: "easy1",
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
            name: "easy2",
            board: [
                [9, 2, 6, 7, 8, 1, 0, 0, 3],
                [8, 0, 0, 4, 3, 9, 6, 2, 0],
                [4, 0, 3, 0, 2, 6, 0, 0, 0],
                [0, 8, 0, 0, 0, 0, 7, 0, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 5],
                [0, 3, 4, 0, 5, 8, 0, 6, 0],
                [1, 0, 7, 0, 6, 0, 4, 5, 2],
                [0, 6, 0, 0, 0, 7, 0, 0, 0],
                [0, 0, 2, 1, 0, 5, 0, 8, 0],
            ],
        },
    ];

    const initial_board = [
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

    // holds the sudoku board, will be a 2d array of numbers, 0 for empty
    let board: number[][] = initial_board;

    let board_is_solvable: boolean = false;
    let problem_text: string = "";

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
                // set the board to the response
                board = response as number[][];
                solving = false;
            })
            .catch((error) => {
                solving = false;
                // alert the user of the error
                problem_text = error;
            });
    }

    async function validate() {
        // validate that the array is 9x9 as Rust will panic if it is too big
        if (board.length != 9 || board[0].length != 9) {
            problem_text = "The board is not 9x9, this should never happen";
            return;
        }
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
        const defaultName = "board.json";
        const filePath = await save({
            defaultPath: (await downloadDir()) + "/" + defaultName,
            filters: [
                {
                    name: "JSON",
                    extensions: ["json"],
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
                    name: "JSON",
                    extensions: ["json"],
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
</script>

<svelte:head>
    <title>Sudoku Solver</title>
</svelte:head>

<div class="container">
    <Check />

    <table class="table table-bordered table-sm table-striped">
        <thead>
            <th></th>
            {#each board as col, i}
                <th>{i + 1}</th>
            {/each}
        </thead>
        <tbody>
            {#each board as row, i}
                <tr>
                    <th>{i + 1}</th>
                    {#each row as cell, j}
                        <td>
                            <input
                                style="width: 100%;"
                                type="number"
                                bind:value={board[i][j]}
                                min="0"
                                max="9"
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
    <button
        class="btn btn-danger"
        on:click={() => {
            board = initial_board;
            board_is_solvable = false;
        }}
    >
        Reset
    </button>
    <button class="btn btn-secondary" on:click={save_board}>
        Save Board
    </button>
    <button class="btn btn-secondary" on:click={load_board}>
        Load Board
    </button>
</div>

<pre>
    board_is_solvable: {board_is_solvable}
    solving: {solving}
    validating: {validating}
    board: {JSON.stringify(board)}
</pre>
