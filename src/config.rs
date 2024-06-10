//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use rust_cli_utils::config::Logging;
//! let config = Logging::new();
//! ```
//! ref: https://github.com/alfredodeza/applied-rust/blob/main/examples/cli-utils/src/config.rs
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use rust_cli_utils::config::Logging;
/// let config = Logging::new();
/// ```
///
/// Creating a new instance of the Logging struct:
/// ```
/// use rust_cli_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}
