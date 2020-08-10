extern crate time;

use std::fmt;

const WIDTH: usize = 9;
const HEIGHT: usize = 9;
const BLOCKSIZE: usize = 3;
const NR_OF_BLOCKS: usize = (WIDTH / BLOCKSIZE) * (HEIGHT / BLOCKSIZE);
const NR_OF_CELLS: usize = WIDTH * HEIGHT;

#[derive(Copy, Clone)]
pub struct Board {
    rows: [u32; HEIGHT],
    columns: [u32; WIDTH],
    blocks: [u32; NR_OF_BLOCKS],
    board: [u32; NR_OF_CELLS],
}

impl Board {
    pub fn from_string(s: String) -> Board {
        debug_assert!(s.len() == 81);

        let mut board = Board {
            rows: [0; HEIGHT],
            columns: [0; WIDTH],
            blocks: [0; NR_OF_BLOCKS],
            board: [0; NR_OF_CELLS],
        };
        let bytes = s.as_bytes();

        for (idx, digit) in bytes.iter().enumerate() {
            let b: u8 = digit - 48;
            board = board.set(idx, b as u32);
        }
        board
    }

    fn is_valid(&self, idx: usize, nr: u32) -> bool {
        debug_assert!(idx < 81);

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
        true
    }

    fn is_occupied(&self, idx: usize) -> bool {
        debug_assert!(idx < 81);
        self.board[idx] != 0
    }

    fn set(&self, idx: usize, nr: u32) -> Board {
        debug_assert!(idx < 81);
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

        ret
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = "".to_string();
        for digit in self.board.iter() {
            ret.push_str(&digit.to_string());
        }

        write!(f, "{}", ret)
    }
}

pub fn solve(b: Board, start_idx: usize) -> Option<Board> {
    if start_idx == NR_OF_CELLS {
        return Some(b);
    }

    if b.is_occupied(start_idx) {
        return solve(b, start_idx + 1);
    }

    let mut nr = 1;
    loop {
        let solved = if b.is_valid(start_idx, nr) {
            let br = b.set(start_idx, nr);
            solve(br, start_idx + 1)
        } else {
            None
        };

        match solved {
            None => {
                nr += 1;
                if nr > 9 {
                    break;
                }
            }
            Some(_) => {
                return solved;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_on_empty() {
        let b = Board::from_string(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        );
        assert!(b.is_valid(0, 1));
    }
    #[test]
    fn not_valid_on_position() {
        let b = Board::from_string(
            "100000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        );
        assert!(!b.is_valid(0, 1));
    }
    #[test]
    fn not_valid_on_row() {
        let b = Board::from_string(
            "010000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        );
        assert!(!b.is_valid(0, 1));
    }
    #[test]
    fn not_valid_on_column() {
        let b = Board::from_string(
            "000000000100000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        );
        assert!(!b.is_valid(0, 1));
    }
    #[test]
    fn not_valid_on_block() {
        let b = Board::from_string(
            "000000000010000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        );
        assert!(!b.is_valid(0, 1));
    }
    #[test]
    fn occupied_on_empty() {
        let b = Board::from_string(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        );
        assert!(!b.is_occupied(0));
    }

    #[test]
    fn not_occupied_on_set() {
        let b = Board::from_string(
            "100000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        );
        assert!(b.is_occupied(0));
    }

    #[test]
    fn occupied_on_set() {
        let b = Board::from_string(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        );
        let b2 = b.set(0, 2);
        assert!(b2.is_occupied(0));
    }

    #[test]
    fn check_solver_1() {
        let solve_string =
            "500063017001008050690000400000057304032010800000206100008001702973080000206004090"
                .to_string();
        let solution_string =
            "584963217321748956697125483169857324732419865845236179458691732973582641216374598"
                .to_string();
        let b = Board::from_string(solve_string);
        let solution = solve(b, 0);

        match solution {
            None => {
                assert!(false);
            }
            Some(b) => assert!(b.to_string() == solution_string),
        }
    }
}
