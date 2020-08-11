extern crate time;
use std::fs::File;
use std::io::{self, BufRead};
use time::Instant;

mod board;

fn main() {
    // File hosts must exist in current path before this produces output
    let file = File::open("sudoku.csv").unwrap();
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut nr = 0;
    let mut max: f64 = 0.0;
    let mut min: f64 = 10000.0;
    for line in lines.skip(1) {
        let z = line.unwrap();
        let mut tokens = z.split(',');
        let puzzle_str = tokens.next().unwrap();
        let solution_str = tokens.next().unwrap();

        let now = Instant::now();

        let solution = board::solve(board::Board::from_str(puzzle_str), 0);
        let elapsed = now.elapsed();
        let f = elapsed.as_seconds_f64();
        if f > max {
            max = f;
        }
        if f < min {
            min = f;
        }

        match solution {
            None => {
                println!("Not solved ! {}", puzzle_str);
            }
            Some(b) => {
                let solution_string = b.to_string();
                if solution_str != solution_string {
                    println!(
                        "wrong solve: {} not equal to {}",
                        solution_str, solution_string
                    );
                }
                print!("{}\r", nr);
                nr += 1;
            }
        }
    }
    println!("Min: {} max: {}", min, max);
}
