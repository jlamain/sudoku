use honggfuzz::fuzz;
use std::io::BufReader;
use std::str;
use stringreader::StringReader;

fn main() {
    loop {
        // The fuzz macro gives an arbitrary object (see `arbitrary crate`)
        // to a closure-like block of code.
        // For performance reasons, it is recommended that you use the native type
        // `&[u8]` when possible.
        // Here, this slice will contain a "random" quantity of "random" data.
        fuzz!(|data: &[u8]| {
            match str::from_utf8(&data)
            {
                Ok(teststr) => {
                    let streader = StringReader::new(teststr);
                    sudoku::read_and_solve(&mut BufReader::new(streader))
                }
                _ => {}
            }
        });
    }
}