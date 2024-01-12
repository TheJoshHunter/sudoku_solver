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
        [0, 0, 0, 0, 0, 0, 0, 0, 8],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let board_is_solvable: boolean = false;

    let thinking: boolean = false; // set to false to show the ready button, set to true to show the thinking spinner

    async function solve() {
        // validate that the array is 9x9 as Rust will panic if it is too big
        if (board.length != 9 || board[0].length != 9) {
            alert("The board is not 9x9");
            return;
        }
        // call the tauri api to solve the sudoku
        thinking = true;
        invoke("solve", { board })
            .then((response) => {
                // set the board to the response
                board = response as number[][];
                // set the thinking spinner to false
                thinking = false;
            })
            .catch((error) => {
                // set the thinking spinner to false
                thinking = false;
                // alert the user of the error
                alert(error);
            });
    }

    async function validate() {
        // validate that the array is 9x9 as Rust will panic if it is too big
        if (board.length != 9 || board[0].length != 9) {
            alert("The board is not 9x9");
            return;
        }
        // call the tauri api to solve the sudoku
        thinking = true;
        invoke("validate", { board })
            .then((response) => {
                // response is a boolean of whether the board is valid or not
                if (response) {
                    alert("The board is valid");
                    board_is_solvable = true;
                } else {
                    alert("The board is not valid");
                    board_is_solvable = false;
                }
            })
            .catch((error) => {
                alert(error);
                board_is_solvable = false;
            });
    }
</script>

<svelte:head>
    <title>Sudoku Solver</title>
</svelte:head>

<div class="container">
    <Check />

    <table>
        {#each board as row, i}
            <tr>
                {#each row as cell, j}
                    <td>
                        <input
                            style="width: 3em"
                            type="number"
                            bind:value={board[i][j]}
                            min="0"
                            max="9"
                        />
                    </td>
                {/each}
            </tr>
        {/each}
    </table>

    <br />

    <button class="btn btn-primary" disabled={thinking} on:click={validate}>
        {#if thinking}
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
        disabled={thinking || board_is_solvable}
        on:click={solve}
    >
        {#if thinking}
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
    thinking: {thinking}
    board: {JSON.stringify(board)}
</pre>
