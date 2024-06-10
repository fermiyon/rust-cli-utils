//! This is a library that provides utilities for CLI
//! # Examples
//! ```rust
//! use rust_cli_utils::read_stdin;
//!
//! let input = read_stdin();
//! println!("You entered: {}", input);
//!
//! ```
//! # Panics
//! The read_stdin function will panic if it fails to read a line from standard input.

use std::io::{BufRead, BufReader};

pub mod colors;
pub mod config;

/// Reads a line from standard input and returns it as a `String`.
///
/// # Examples
///
/// ```
/// use rust_cli_utils::read_stdin;
///
///
/// let input = read_stdin();
/// println!("You entered: {}", input);
///
/// ```

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_stdin() {
        let input = "Hello, world!\n";
        let expected_output = "Hello, world!";
        let mut reader = Cursor::new(input.as_bytes());
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_read_stdin_empty() {
        let input = "";
        let expected_output = "";
        let mut reader = Cursor::new(input.as_bytes());
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output);
    }
}
