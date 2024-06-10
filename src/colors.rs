//! ANSI color codes for use in terminal output.
//! # Examples:
//! ```
//! use rust_cli_utils::colors:{red,blue};
//! let red_string = red("Hello");
//! let blue_string = blue("Hello");
//! ```

/// Returns a string wrapped in ANSI red color codes
/// # Examples:
/// ```
/// use rust_cli_utils::colors::red;
/// let s = red("Hello");
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}
