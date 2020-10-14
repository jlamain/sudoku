extern crate time;
use std::fs::File;
use std::io::{self, BufRead};
use time::Instant;
use rayon::prelude::*;

mod board;

fn try_solve( puzzle_str: &str, solution_str: &str)
{
    let solution = board::solve(board::Board::from_str(puzzle_str), 0);

    match solution {
        None => {
            println!("Not solved ! {}", puzzle_str);
        }
        Some(b) => {
            let solution_string = &b.to_string();
            if solution_str != solution_string {
                println!(
                    "wrong solve: {} not equal to {}",
                    solution_str, solution_string
                );
            }
        }
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    let file = File::open("sudoku.csv").unwrap();
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut v_all: Vec<(String, String)> = Vec::new();

    for line in lines.skip(1) {
        let z = line.unwrap();
        let mut tokens = z.split(',');
        v_all.push( (tokens.next().unwrap().to_string(), tokens.next().unwrap().to_string()));
    }
    let now = Instant::now();

    v_all.par_iter().for_each(|puzzle|   try_solve(&puzzle.0, &puzzle.1) );

    println!("Solution time: {} seconds ", now.elapsed().as_seconds_f64());
}
