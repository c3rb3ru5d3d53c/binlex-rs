use std::io::ErrorKind;
use std::io::{self, BufRead, IsTerminal, Write};
use std::fmt::Display;
use std::process;
pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
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