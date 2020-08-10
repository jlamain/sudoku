extern crate time;
use std::fs::File;
use std::io::{self, BufRead};
use time::Instant;

const WIDTH: usize = 9;
const HEIGHT: usize = 9;
const BLOCKSIZE: usize = 3;
const NR_OF_BLOCKS: usize = (WIDTH / BLOCKSIZE) * (HEIGHT / BLOCKSIZE);
const NR_OF_CELLS: usize = WIDTH * HEIGHT;

#[derive(Copy, Clone)]
struct Board {
    rows: [u32; HEIGHT],
    columns: [u32; WIDTH],
    blocks: [u32; NR_OF_BLOCKS],
    board: [u32; NR_OF_CELLS],
}

impl Board {
    fn from_string(s: String) -> Board {
        let mut board = Board {
            rows: [0; HEIGHT],
            columns: [0; WIDTH],
            blocks: [0; NR_OF_BLOCKS],
            board: [0; NR_OF_CELLS],
        };
        let bytes = s.as_bytes();
        for digit in 0..s.len() {
            let b: u8 = bytes[digit] - 48;
            board = board.set(digit, b as u32);
        }

        return board;
    }

    fn is_valid(&self, idx: usize, nr: u32) -> bool {
        let y = idx / HEIGHT;
        let bitvalue: u32 = 1 << nr;
        if (self.rows[y] & bitvalue) != 0 {
            return false;
        }

        let x = idx % WIDTH;
        if (self.columns[x] & bitvalue) != 0 {
            return false;
        }
        let x3: usize = x / BLOCKSIZE;
        let y3: usize = y / BLOCKSIZE;
        let off: usize = y3 * BLOCKSIZE + x3;
        if (self.blocks[off] & bitvalue) != 0 {
            return false;
        }
        return true;
    }

    fn is_occupied(&self, idx: usize) -> bool {
        return self.board[idx] != 0;
    }

    fn set(&self, idx: usize, nr: u32) -> Board {
        let y = idx / HEIGHT;
        let x = idx % WIDTH;

        let mut ret = *self;
        ret.rows[y] |= 1 << nr;
        ret.columns[x] |= 1 << nr;

        let x3: usize = x / BLOCKSIZE;
        let y3: usize = y / BLOCKSIZE;
        let off: usize = y3 * BLOCKSIZE + x3;
        ret.blocks[off] |= 1 << nr;
        ret.board[idx] = nr;

        return ret;
    }
    fn to_string(&self) -> String {
        let mut ret = "".to_string();
        for digit in self.board.iter() {
            ret.push_str(&digit.to_string());
        }
        return ret;
    }
}

fn solve(b: Board, start_idx: usize) -> Option<Board> {
    if start_idx == NR_OF_CELLS {
        return Some(b);
    }

    if b.is_occupied(start_idx) {
        return solve(b, start_idx + 1);
    }

    let mut nr = 1;
    loop {
        let mut solved: Option<Board> = None;

        if b.is_valid(start_idx, nr) {
            let br = b.set(start_idx, nr);
            solved = solve(br, start_idx + 1);
        }

        match solved {
            None => {
                nr = nr + 1;
                if nr > 9 {
                    break;
                }
            }
            Some(_) => {
                return solved;
            }
        }
    }
    return None;
}

fn main() {
    // File hosts must exist in current path before this produces output
    let file = File::open("sudoku.csv").unwrap();
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    
    let mut nr = 0;
    let mut max:f64 = 0.0;
    let mut min:f64 = 10000.0;
    for line in lines.skip(1) {
        let z = line.unwrap();
        let tokens: Vec<&str> = z.split(",").collect();

        let now = Instant::now();

        let solution = solve(Board::from_string(tokens[0].to_string()), 0);
        let elapsed = now.elapsed();
        let f = elapsed.as_seconds_f64();
        if f > max {
            max = f;
        }
        if f < min {
            min = f;
        }
        
        let solution_file = tokens[1].to_string();
        match solution {
            None => {
                println!("Not solved ! {}", tokens[0].to_string());
            }
            Some(b) => {
                let solution_string = b.to_string();
                if solution_file != solution_string {
                    println!(
                        "wrong solve: {} not equal to {}",
                        solution_file, solution_string
                    );
                }
                print!("{}\r", nr);
                nr = nr + 1;
            }
        }
    }
    println!("Min: {} max: {}", min, max);
}
