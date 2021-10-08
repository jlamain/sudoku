use rayon::prelude::*;
use std::io::{self, BufRead};
use time::Instant;

mod board;

fn try_solve(puzzle_str: &str, solution_str: &str) {
    if solution_str.contains('0') {
        eprintln!("Solution must not contain 0's");
        return;
    }

    if let (Some(puzzle), Some(solution)) = (
        board::Board::from_str(puzzle_str),
        board::Board::from_str(solution_str),
    ) {
        let solution_solved = board::solve(puzzle, 0);

        match solution_solved {
            None => {
                println!("Not solved ! {}", puzzle_str);
            }
            Some(solution_solved) => {
                if solution != solution_solved {
                    println!(
                        "wrong solve: {} not equal to {}",
                        solution.to_string(),
                        solution_solved.to_string()
                    );
                    let a1 = board::Board::from_str(&solution.to_string());
                    let a2 = board::Board::from_str(&solution_solved.to_string());

                    if a1 == None || a2 == None {
                        eprintln!("Huh");
                    }
                }
            }
        }
    }
}

pub fn read_and_solve(reader: &mut dyn io::BufRead) {
    let lines = reader.lines();
    let mut v_all: Vec<(String, String)> = Vec::new();

    for line in lines.skip(1) {
        let z = line.unwrap();
        let mut tokens = z.split(',');
        let puzzle = tokens.next();
        let solution = tokens.next();

        if let (Some(p), Some(s)) = (puzzle, solution) {
            v_all.push((p.to_string(), s.to_string()));
        }
    }
    let now = Instant::now();

    v_all
        .par_iter()
        .for_each(|puzzle| try_solve(&puzzle.0, &puzzle.1));

    println!("Solution time: {} seconds ", now.elapsed().as_seconds_f64());
}
