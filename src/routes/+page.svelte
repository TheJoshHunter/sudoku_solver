<script lang="ts">
    import { browser } from "$app/environment";
    import Greet from "$lib/Greet.svelte";
    import { onMount } from "svelte";
    import "../../node_modules/bootstrap/dist/css/bootstrap.css";
    onMount(() => {
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

    let user_is_ready: boolean = false;

    let thinking: boolean = false; // set to false to show the ready button, set to true to show the thinking spinner

    let last_move: string = ""; // holds the last move made by the computer for the user to see

    function validate() {
        // starting from the top left, check each row, column, and 3x3 square for duplicates
        // if there are duplicates, return false
        // if there are no duplicates, return true
        // we dont care about zeroes because they are empty squares
        board.forEach((row, row_index) => {
            // check for duplicates in the row
            row.forEach((num) => {
                if (row.filter((n) => n === num).length > 1) return false;
            });
            // check for duplicates in the column
            board.forEach((r) => {
                if (r.filter((n, i) => n === row[i]).length > 1) return false;
            });
            // check for duplicates in the 3x3 square (remember to make the squares based on the index of the row and column)
            if (row_index <= 0 || row_index >= 2) {
                // check the top 3 rows
                if (row_index <= 0) {
                    // check the top left 3x3 square
                    if (row_index <= 0) {
                        // check the top left 3x3 square
                    }
                    if (row_index <= 0) {
                        // check the top middle 3x3 square
                    }
                    if (row_index <= 0) {
                        // check the top right 3x3 square
                    }
                }
                if (row_index >= 2) {
                    // check the bottom 3 rows
                    if (row_index >= 2) {
                        // check the bottom left 3x3 square
                    }
                    if (row_index >= 2) {
                        // check the bottom middle 3x3 square
                    }
                    if (row_index >= 2) {
                        // check the bottom right 3x3 square
                    }
                }
            }
        });
        // if we get here, we're done
        return true;
    }

    function arm() {
        // called when the ready button is clicked
        user_is_ready = true;
    }

    function solve() {
        // called when the solve button is clicked
        // uses simple backtracking to solve the sudoku board
        // algorithm:
        // 1. find the first empty square
        // 2. try each number from 1 to 9 in that square
        // 3. if the number is valid, move on to the next empty square
        // 4. if the number is not valid, try the next number
        // 5. if all numbers are not valid, go back to the previous square and try the next number
        // 6. if all numbers are not valid and there is no previous square, the board is unsolvable
        // 7. if all squares are filled, the board is solved
        // 8. if the board is solved, return true

        // First make sure that we havent already solved the board saving everyone some time
        if (validate()) return true;

        // evidently we havent solved the board yet, so lets get to work
        // find the first empty square
        let empty_square: number[] = [0, 0];
        board.forEach((row, row_index) => {
            row.forEach((num, col_index) => {
                if (num === 0) {
                    empty_square = [row_index, col_index];
                    return;
                }
            });
        });
        // now that we have the first empty square, lets try each number from 1 to 9
        for (let i = 1; i <= 9; i++) {
            // try the number
            board[empty_square[0]][empty_square[1]] = i;
            // check if the number is valid
            if (validate()) {
                // the number is valid, so lets move on to the next empty square
                if (solve()) {
                    // the board is solved, so lets return true
                    return true;
                } else {
                    // the board is not solved, so lets try the next number
                    continue;
                }
            } else {
                // the number is not valid, so lets try the next number
                continue;
            }
        }
    }
</script>

<svelte:head>
    <title>Sudoku Solver</title>
</svelte:head>

<div class="container">
    <Greet />
</div>
