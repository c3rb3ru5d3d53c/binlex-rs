use std::io::ErrorKind;
use std::io::{self, BufRead, IsTerminal, Write};
use std::fmt::Display;
use std::process;

/// Represents a wrapper for standard input operations.
pub struct Stdin;

/// Represents a wrapper for standard output operations.
pub struct Stdout;

/// Represents a wrapper for standard error operations.
pub struct Stderr;

impl Stdin {
    /// Reads lines from standard input and writes each line to standard output.
    ///
    /// This function reads lines from standard input if it's not a terminal,
    /// locking the input for safe handling in buffered mode. If a line is read
    /// successfully, it's printed using `Stdout`. If an error occurs, it prints
    /// the error message and exits with a non-zero status code.
    #[allow(dead_code)]
    pub fn passthrough() {
        let stdin = io::stdin();
        if !stdin.is_terminal() {
            let handle = stdin.lock();
            for line in handle.lines() {
                match line {
                    Ok(line) => {
                        Stdout.print(line);
                    },
                    Err(error) => {
                        eprintln!("{}", error);
                        process::exit(1);
                    },
                }
            }
        }
    }
    /// Prints a line to standard error.
    ///
    /// # Arguments
    ///
    /// * `line` - The line to be printed, which implements the `Display` trait.
    ///
    /// If an error occurs, this method checks if it was due to a broken pipe.
    /// If it was, the program exits with code `0`. For other errors, it logs
    /// an error message to standard error and exits with code `1`.
    #[allow(dead_code)]
    pub fn print<T: Display>(&self, line: T) {
        writeln!(io::stderr(), "{}", line).unwrap_or_else(|e| {
            if e.kind() == ErrorKind::BrokenPipe {
                std::process::exit(0);
            } else {
                eprintln!("error writing to stdout: {}", e);
                std::process::exit(1);
            }
        });
    }
}

impl Stdout {
    /// Prints a line to standard output.
    ///
    /// # Arguments
    ///
    /// * `line` - The line to be printed, which implements the `Display` trait.
    ///
    /// If an error occurs, this method checks if it was due to a broken pipe.
    /// If it was, the program exits with code `0`. For other errors, it logs
    /// an error message to standard error and exits with code `1`.

    #[allow(dead_code)]
    pub fn print<T: Display>(&self, line: T) {
        writeln!(io::stdout(), "{}", line).unwrap_or_else(|e| {
            if e.kind() == ErrorKind::BrokenPipe {
                std::process::exit(0);
            } else {
                eprintln!("error writing to stdout: {}", e);
                std::process::exit(1);
            }
        });
    }
}

impl Stderr {
    /// Prints a line to standard error.
    ///
    /// # Arguments
    ///
    /// * `line` - The line to be printed, which implements the `Display` trait.
    ///
    /// If an error occurs, this method checks if it was due to a broken pipe.
    /// If it was, the program exits with code `0`. For other errors, it logs
    /// an error message to standard error and exits with code `1`.
    #[allow(dead_code)]
    pub fn print<T: Display>(&self, line: T) {
        writeln!(io::stderr(), "{}", line).unwrap_or_else(|e| {
            if e.kind() == ErrorKind::BrokenPipe {
                std::process::exit(0);
            } else {
                eprintln!("error writing to stdout: {}", e);
                std::process::exit(1);
            }
        });
    }
}