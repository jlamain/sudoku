# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Sudoku solver written in Rust focused on performance and correctness. Uses a recursive depth-first search with MRV (Minimum Remaining Values) heuristic, naked singles propagation, and a mutable board with bitfield-based validation.

## Development Commands

### Build and Test
```bash
# Build the project
cargo build --verbose

# Build optimized release version with profiling support
cargo build --release

# Run all tests (includes unit tests in src/board.rs and src/lib.rs)
cargo test --verbose

# Run a specific test
cargo test test_name

# Run the solver with default CSV (sudoku.csv)
cargo run --release

# Run with custom CSV file
cargo run --release path/to/puzzles.csv
```

### Linting
```bash
# Run clippy with pedantic and nursery lints (project uses strict linting)
cargo clippy --all-targets --all-features -- -D warnings
```

### Fuzzing
The project includes three fuzzing implementations:
```bash
# libfuzzer (requires nightly)
./fuzz-libfuzz.sh

# honggfuzz
./fuzz-honggfuzz.sh

# AFL
./fuzz-afl.sh
```

Test inputs are in `tests/` directory for regression testing.

## Architecture

### Core Data Structure: `Board` (src/board.rs)

The `Board` struct is the heart of the solver, designed for performance:

- **Mutable design with backtracking**: `set(idx, nr)` and `unset(idx, nr)` modify in-place; the solver undoes changes on backtrack
- **Bitfield optimization**: Uses `u16` bitfields to track which numbers (1-9) are present in each row, column, and 3x3 block
  - `rows: [u16; 9]` - one bitfield per row
  - `columns: [u16; 9]` - one bitfield per column
  - `blocks: [u16; 9]` - one bitfield per 3x3 block
  - `cells: [u8; 81]` - stores the actual cell values (0-9) as `u8` for cache efficiency
- **Fast validation**: Checking if a number can be placed requires only bitwise operations on 3 bitfields (O(1) instead of scanning arrays)
- **Bitwise candidate computation**: `candidates(idx)` returns a bitmask of all valid numbers in one OR+AND operation; candidates are iterated with `trailing_zeros` + bit clearing

### Key Operations

- `Board::from_str()` - Parses 81-character string (digits 0-9, where 0 = empty), uses `bytes()` for efficiency
- `Board::is_valid(idx, nr)` - Checks if number `nr` can be placed at position `idx`
- `Board::candidates(idx)` - Returns bitmask of all valid numbers for a cell
- `Board::set(idx, nr)` / `Board::unset(idx, nr)` - Mutate cell and update bitfields in-place
- `solve_puzzle(board)` - Entry point: collects empty cells, then calls `solve_mrv`
- `solve_mrv(board, empties, start)` - Recursive DFS with MRV heuristic and naked singles propagation

### Parsing and Display

- Implements `FromStr` trait for parsing strings to `Board`
- Implements `From<Board> for String` for conversion (not `to_string()`)
- Implements `Display` trait which delegates to `String::from()`
- Board strings are always exactly 81 ASCII digits (0-9)

### Solver Entry Point (src/lib.rs)

- `read_and_solve()` - Main function that reads CSV file with format: `puzzle,solution`
- CSV format: first row is header, subsequent rows are puzzle/solution pairs
- Validates solutions don't contain '0' before attempting to solve

### Solver Algorithm

The solver uses a combination of techniques from CSP (Constraint Satisfaction Problem) theory:

1. **Pre-computed empty cell list** - collected once before solving; avoids scanning all 81 cells on every recursive call
2. **Naked singles propagation** - before branching, fills all cells with exactly one candidate; repeat until no more exist; undo on backtrack
3. **MRV (Minimum Remaining Values) heuristic** - branches on the empty cell with the fewest candidates, pruning the search tree aggressively; from Russell & Norvig's "Artificial Intelligence: A Modern Approach"

These techniques reduce the solve time from ~9s to ~2.85s on the standard sudoku.csv benchmark.

### Performance Considerations

- Uses `#[inline(always)]` on hot path functions (`is_valid`, `is_occupied`, `set`, `unset`, `candidates`)
- Release profile includes line tables for profiling (`debug = "line-tables-only"`)
- Frame pointers enabled for better profiling (`force-frame-pointers=yes`)

## Testing Strategy

- Comprehensive unit tests in `src/board.rs` covering:
  - Invalid inputs (wrong length, non-ASCII, duplicate numbers)
  - Validation logic (row/column/block conflicts)
  - Board equality and conversion
  - Solver correctness (includes Norvig hard puzzles)
- Integration tests in `src/lib.rs` for CSV parsing
- Test files in `tests/` directory for fuzzing regression

## Code Style

- Uses Rust 2024 edition
- Clippy with pedantic and nursery lints enabled
- Prefers `From` trait over `to_string()` for conversions
- Uses `const fn` where possible for compile-time evaluation
- Explicit `#[allow(clippy::cast_possible_truncation)]` where casts are safe