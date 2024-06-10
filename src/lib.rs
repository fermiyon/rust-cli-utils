//! This is a library that provides utilities for CLI
//! # Examples
//! ```rust
//! use cli_utils::read_stdin;
//! fn main() {
//!     let input = read_stdin();
//!     println!("You entered: {}", input);
//! }
//! ```
//! # Panics
//! The read_stdin function will panic if it fails to read a line from standard input.

use std::io::{BufRead, BufReader};

/// Reads a line from standard input and returns it as a `String`.
///
/// # Examples
///
/// ```rust
/// fn main() {
///     let input = read_stdin();
///     println!("You entered: {}", input);
/// }
/// ```

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut stdin = BufReader::new(stdin.lock());
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read line");
    input
}
