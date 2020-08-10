# sudoku
Sudoko solver in rust

This is my personal rust solver to help me learn the Rust programming language.

It calculates the solutions for 1000000 soduku puzzles. The test set can be found at: https://www.kaggle.com/bryanpark/sudoku
Download the csv file from there and rename to soduku.csv.

The algorithm does a recursive depth first search. 
The board structure is an immutable data structure. When setting a cell a new board is returned.

The board structure contains the 'board' cells.
it also contains extra members to make lookup faster, so we do not need to scan if a number is in a row, column, or block. 
For each row, column and block a uint16 is stored. In this uint16 we store a bit when a specific number is set.
