extern crate time;
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use time::Instant;

mod board;

fn try_solve(puzzle_str: &str, solution_str: &str) {
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
    let args: Vec<String> = env::args().collect();

    let file = if args.len() > 1 { File::open(args[1].as_str()) } else { File::open("sudoku.csv")};
    let reader = io::BufReader::new(file.unwrap());
    let lines = reader.lines();
    let mut v_all: Vec<(String, String)> = Vec::new();

    for line in lines.skip(1) {
        let z = line.unwrap();
        let mut tokens = z.split(',');
        let puzzle = tokens.next();
        let solution = tokens.next();
        match (puzzle, solution)
        {
            (Some(p), Some(s)) => {
                v_all.push((
                    p.to_string(),
                    s.to_string(),
                ));
            }
            _ => (), // ignore empty strings
        }

    }
    let now = Instant::now();

    v_all
        .par_iter()
        .for_each(|puzzle| try_solve(&puzzle.0, &puzzle.1));

    println!("Solution time: {} seconds ", now.elapsed().as_seconds_f64());
}
