<script lang="ts">
    import { browser } from "$app/environment";
    import Check from "$lib/Check.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import "../../node_modules/bootstrap/dist/css/bootstrap.css";
    onMount(() => {
        // wait till were in the browser to import bootstrap
        if (browser)
            import(
                "../../node_modules/bootstrap/dist/js/bootstrap.js" as string
            );
    });

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

    let solving: boolean = false;
    let validating: boolean = false;

    async function solve() {
        // validate that the array is 9x9 as Rust will panic if it is too big
        if (board.length != 9 || board[0].length != 9) {
            alert("The board is not 9x9");
            return;
        }
        // call the tauri api to solve the sudoku
        solving = true;
        invoke("solve", { board })
            .then((response) => {
                // set the board to the response
                board = response as number[][];
                // set the thinking spinner to false
                solving = false;
            })
            .catch((error) => {
                // set the thinking spinner to false
                solving = false;
                // alert the user of the error
                problem_text = error;
            });
    }

    async function validate() {
        // validate that the array is 9x9 as Rust will panic if it is too big
        if (board.length != 9 || board[0].length != 9) {
            alert("The board is not 9x9");
            return;
        }
        // call the tauri api to solve the sudoku
        validating = true;
        invoke("validate", { board })
            .then((response) => {
                // response is a boolean of whether the board is valid or not
                if (response) {
                    alert("The board is valid");
                    board_is_solvable = true;
                    problem_text = ""; // make the problem text blank
                } else {
                    problem_text = "The board is not valid";
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

    <button class="btn btn-primary" disabled={validating} on:click={validate}>
        {#if validating}
            <span
                class="spinner-border spinner-border-sm"
                role="status"
                aria-hidden="true"
            ></span>
            Validating...
        {:else}
            Validate Board (required to solve)
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
</div>

<pre>
    board_is_solvable: {board_is_solvable}
    thinking: {solving}
    validating: {validating}
    board: {JSON.stringify(board)}
</pre>
