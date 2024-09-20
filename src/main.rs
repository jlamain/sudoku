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

    if let Ok(file) = File::open(name) {
        read_and_solve(&mut io::BufReader::new(file));
    } else {
        eprintln!("Could not open file {name}");
        std::process::exit(1);
    };
}
