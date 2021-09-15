use afl::fuzz;
use std::io::BufReader;
use std::str;
use stringreader::StringReader;

fn main() {
    fuzz!(|data: &[u8]| {
        match str::from_utf8(&data) {
            Ok(teststr) => {
                let streader = StringReader::new(teststr);
                sudoku::read_and_solve(&mut BufReader::new(streader))
            }
            _ => {}
        }
    });
}
