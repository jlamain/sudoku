use rayon::prelude::*;
use std::io::{self, BufRead};
use std::time::Instant;

mod board;

fn try_solve(puzzle_str: &str, solution_str: &str) {
    if solution_str.contains('0') {
        return;
    }

    if let (Ok(puzzle), Ok(solution)) = (
        puzzle_str.parse::<board::Board>(),
        solution_str.parse::<board::Board>(),
    ) {
        let solution_solved = board::solve(puzzle, 0);

        match solution_solved {
            None => {
                println!("Not solved ! {puzzle_str}");
            }
            Some(solution_solved) => {
                if solution != solution_solved {
                    println!("wrong solve: {solution} not equal to {solution_solved}");
                    let a1 = String::from(solution).parse::<board::Board>();
                    let a2 = String::from(solution_solved).parse::<board::Board>();

                    if a1.is_ok() || a2.is_ok() {
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

    for line in lines.skip(1).flatten() {
        let mut tokens = line.split(',');
        let puzzle = tokens.next();
        let solution = tokens.next();

        if let (Some(p), Some(s)) = (puzzle, solution) {
            v_all.push((p.to_owned(), s.to_owned()));
        }
    }
    let now = Instant::now();

    v_all
        .par_iter()
        .for_each(|puzzle| try_solve(&puzzle.0, &puzzle.1));

    println!("Solution time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_and_solve_with_valid_csv() {
        let csv_data = "puzzle,solution\n000000000000000000000000000000000000000000000000000000000000000000000000000000000,123456789456789123789123456234567891567891234891234567345678912678912345912345678\n";
        let mut cursor = Cursor::new(csv_data);
        
        // This should not panic
        read_and_solve(&mut cursor);
    }

    #[test]
    fn test_read_and_solve_with_empty_csv() {
        let csv_data = "puzzle,solution\n";
        let mut cursor = Cursor::new(csv_data);
        
        // This should not panic
        read_and_solve(&mut cursor);
    }

    #[test]
    fn test_read_and_solve_with_invalid_lines() {
        let csv_data = "puzzle,solution\ninvalid_line_without_comma\n000000000000000000000000000000000000000000000000000000000000000000000000000000000,123456789456789123789123456234567891567891234891234567345678912678912345912345678\n";
        let mut cursor = Cursor::new(csv_data);
        
        // This should not panic
        read_and_solve(&mut cursor);
    }

    #[test]
    fn test_try_solve_with_solution_containing_zeros() {
        // This should return early without attempting to solve
        try_solve("000000000000000000000000000000000000000000000000000000000000000000000000000000000", "000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    }
}