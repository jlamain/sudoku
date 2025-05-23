use std::fmt;
type BitField = u16;

use std::str::FromStr;

const WIDTH: usize = 9;
const HEIGHT: usize = 9;
const BLOCKSIZE: usize = 3;
const NR_OF_BLOCKS: usize = (WIDTH / BLOCKSIZE) * (HEIGHT / BLOCKSIZE);
const NR_OF_CELLS: usize = WIDTH * HEIGHT;

#[derive(Copy, Clone, Debug)]
pub struct Board {
    rows: [BitField; HEIGHT],
    columns: [BitField; WIDTH],
    blocks: [BitField; NR_OF_BLOCKS],
    cells: [BitField; NR_OF_CELLS],
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.cells == other.cells
    }
}

impl Eq for Board {}
#[derive(Debug, PartialEq)]
pub struct BoardConversionError;

impl std::fmt::Display for BoardConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid board format")
    }
}

impl std::error::Error for BoardConversionError {}

impl FromStr for Board {
    type Err = BoardConversionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 81 {
            return Err(BoardConversionError);
        }

        if !s.is_ascii() {
            return Err(BoardConversionError);
        }

        let mut board = Self {
            rows: [0; HEIGHT],
            columns: [0; WIDTH],
            blocks: [0; NR_OF_BLOCKS],
            cells: [0; NR_OF_CELLS],
        };

        for (idx, digit) in s.chars().enumerate() {
            if !digit.is_ascii_digit() {
                return Err(BoardConversionError);
            }
            #[allow(clippy::cast_possible_truncation)]
            let b: BitField = digit.to_digit(10).unwrap() as BitField;
            if b != 0 {
                if board.is_valid(idx, b as BitField) {
                    board = board.set(idx, b as BitField);
                } else {
                    return Err(BoardConversionError);
                }
            }
        }
        Ok(board)
    }
}

impl Board {
    const fn is_valid(&self, idx: usize, nr: BitField) -> bool {
        let y = idx / HEIGHT;
        let bitvalue: BitField = 1 << nr;
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

    const fn is_occupied(&self, idx: usize) -> bool {
        self.cells[idx] != 0
    }

    const fn set(&self, idx: usize, nr: BitField) -> Self {
        let y = idx / HEIGHT;
        let x = idx % WIDTH;

        let mut ret = *self;
        ret.rows[y] |= 1 << nr;
        ret.columns[x] |= 1 << nr;

        let x3: usize = x / BLOCKSIZE;
        let y3: usize = y / BLOCKSIZE;
        let off: usize = y3 * BLOCKSIZE + x3;
        ret.blocks[off] |= 1 << nr;
        ret.cells[idx] = nr;

        ret
    }
}

impl From<Board> for String {
    fn from(board: Board) -> Self {
        let mut ret = String::with_capacity(NR_OF_CELLS);
        for &digit in &board.cells {
            ret.push((b'0' + digit as u8) as char);
        }
        ret
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

pub fn solve(b: Board, start_idx: usize) -> Option<Board> {
    if start_idx == NR_OF_CELLS {
        return Some(b);
    }

    if b.is_occupied(start_idx) {
        return solve(b, start_idx + 1);
    }

    for nr in 1..=9 {
        if b.is_valid(start_idx, nr) {
            let br = b.set(start_idx, nr);
            if let Some(solution) = solve(br, start_idx + 1) {
                return Some(solution);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_length() {
        let b = Board::from_str(
            "0000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(b.is_err());
    }
    #[test]
    fn invalid_chars() {
        let b = Board::from_str(
            "ü0000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(b.is_err());
    }
    #[test]
    fn invalid_board_duplicate_numbers() {
        let b = Board::from_str(
            "110000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(b.is_err());
    }
    #[test]
    fn valid_on_empty() {
        let b = Board::from_str(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(b.unwrap().is_valid(0, 1));
    }

    #[test]
    fn board_equal() {
        let b = Board::from_str(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        let c = Board::from_str(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(b == c);
    }

    #[test]
    fn board_not_equal() {
        let b = Board::from_str(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        let c = Board::from_str(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000001",
        );
        assert!(b != c);
    }
    #[test]
    fn not_valid_on_position() {
        let b = Board::from_str(
            "100000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(!b.unwrap().is_valid(0, 1));
    }
    #[test]
    fn not_valid_on_row() {
        let b = Board::from_str(
            "010000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(!b.unwrap().is_valid(0, 1));
    }
    #[test]
    fn not_valid_on_column() {
        let b = Board::from_str(
            "000000000100000000000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(!b.unwrap().is_valid(0, 1));
    }
    #[test]
    fn not_valid_on_block() {
        let b = Board::from_str(
            "000000000010000000000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(!b.unwrap().is_valid(0, 1));
    }
    #[test]
    fn occupied_on_empty() {
        let b = Board::from_str(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(!b.unwrap().is_occupied(0));
    }

    #[test]
    fn not_occupied_on_set() {
        let b = Board::from_str(
            "100000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(b.unwrap().is_occupied(0));
    }

    #[test]
    fn occupied_on_set() {
        let b = Board::from_str(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        let b2 = b.unwrap().set(0, 2);
        assert!(b2.is_occupied(0));
    }

    #[test]
    fn check_solver_1() {
        let solve_string =
            "500063017001008050690000400000057304032010800000206100008001702973080000206004090";
        let solution_string =
            "584963217321748956697125483169857324732419865845236179458691732973582641216374598";
        let b = Board::from_str(solve_string).unwrap();
        let solution = solve(b, 0);
        assert!(String::from(solution.unwrap()) == solution_string);
    }

    #[test]
    fn test_from_board_to_string() {
        let board_str = "000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let board = Board::from_str(board_str).unwrap();
        assert_eq!(String::from(board), board_str);
    }

    #[test]
    fn test_from_board_to_string_with_values() {
        let board_str = "123456789000000000000000000000000000000000000000000000000000000000000000000000000";
        let board = Board::from_str(board_str).unwrap();
        assert_eq!(String::from(board), board_str);
    }

    #[test]
    fn test_board_debug_format() {
        let board_str = "000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let board = Board::from_str(board_str).unwrap();
        let debug_str = format!("{:?}", board);
        assert!(debug_str.contains("Board"));
    }

    #[test]
    fn test_error_display() {
        let error = BoardConversionError;
        assert_eq!(error.to_string(), "Invalid board format");
    }

    #[test]
    fn test_board_equality() {
        let board1 = Board::from_str("000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let board2 = Board::from_str("000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
        assert_eq!(board1, board2);
    }

    #[test]
    fn test_invalid_characters_in_board() {
        let invalid_boards = [
            "abcdefghi000000000000000000000000000000000000000000000000000000000000000000000000000",
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000!",
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000-1",
        ];
        
        for invalid_board in &invalid_boards {
            assert!(Board::from_str(invalid_board).is_err());
        }
    }

    #[test]
    fn test_solve_empty_board() {
        let empty_board = Board::from_str("000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let solution = solve(empty_board, 0);
        assert!(solution.is_some());
        
        let solved = solution.unwrap();
        let solved_str = String::from(solved);
        assert!(!solved_str.contains('0'));
    }

    #[test]
    fn test_impossible_board() {
        let impossible_board = "111000000000000000000000000000000000000000000000000000000000000000000000000000000";
        assert!(Board::from_str(impossible_board).is_err());
    }

    #[test]
    fn norvig_hard() {
        assert!(solve(
            Board::from_str(
                "400000805030000000000700000020000060000080400000010000000603070500200000104000000"
            )
            .unwrap(),
            0
        )
        .is_some());
        assert!(solve(
            Board::from_str(
                "052006800000007020000000600004800900200410000001000008006100380000090006300600109"
            )
            .unwrap(),
            0
        )
        .is_some());
        assert!(solve(
            Board::from_str(
                "000010780500009000000000040020000000000600003074080000000003002080040010600500000"
            )
            .unwrap(),
            0
        )
        .is_some());
        assert!(solve(
            Board::from_str(
                "100000003060300700070005001210700090007000000008010020000806400009020060000400000"
            )
            .unwrap(),
            0
        )
        .is_some());
        assert!(solve(
            Board::from_str(
                "400070100001904605000001000000700002002030000847006000014000806020000300600090000"
            )
            .unwrap(),
            0
        )
        .is_some());
        assert!(solve(
            Board::from_str(
                "000000801700200000000506000000700050010000300080000000500000020030080000600040000"
            )
            .unwrap(),
            0
        )
        .is_some());
        assert!(solve(
            Board::from_str(
                "963000000100008000000205000040800000010000700000030025700000030009020407000000900"
            )
            .unwrap(),
            0
        )
        .is_some());
        assert!(solve(
            Board::from_str(
                "150300000070040200004072000008000000000900108010080790000003800000000000600007423"
            )
            .unwrap(),
            0
        )
        .is_some());
        assert!(solve(
            Board::from_str(
                "000075000010020000040003000500000302000800010000000600000100480200000000700000000"
            )
            .unwrap(),
            0
        )
        .is_some());
        assert!(solve(
            Board::from_str(
                "600000703040800000000000000000504080700200000103000000020000050000070900000010000"
            )
            .unwrap(),
            0
        )
        .is_some());
    }
}
