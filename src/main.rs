extern crate time;
use std::env;
use std::fs::File;
use std::io::{self};
use sudoku::read_and_solve;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = if args.len() > 1 {
        args[1].as_str()
    } else {
        "sudoku.csv"
    };

    match File::open(name) {
        Ok(file) => read_and_solve(&mut io::BufReader::new(file)),
        Err(_) => {
            eprintln!("Could not open file {}", name);
            std::process::exit(1);
        }
    };
}
