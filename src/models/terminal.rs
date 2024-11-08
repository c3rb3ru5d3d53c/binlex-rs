use std::io::{Error, ErrorKind};
use std::io::{self, BufRead, IsTerminal, Write};

pub struct Terminal;

impl Terminal {
    #[allow(dead_code)]
    pub const STDIN: Stdin = Stdin;
    pub const STDOUT: Stdout = Stdout;
}

pub struct Stdin;
pub struct Stdout;

impl Stdin {
    #[allow(dead_code)]
    pub fn passthrough(&self) -> Result<(), Error> {
        let stdin = io::stdin();
        if !stdin.is_terminal() {
            let handle = stdin.lock();
            for line in handle.lines() {
                match line {
                    Ok(line) => {
                        Terminal::STDOUT.print(line)?;
                    },
                    Err(error) => return Err(error),
                }
            }
        }
        Ok(())
    }
}

impl Stdout {
    #[allow(dead_code)]
    pub fn print(&self, line: String) -> Result<(), Error> {
        writeln!(io::stdout(), "{}", line).unwrap_or_else(|e| {
            if e.kind() == ErrorKind::BrokenPipe {
                std::process::exit(0);
            } else {
                eprintln!("error writing to stdout: {}", e);
                std::process::exit(1);
            }
        });
        Ok(())
    }
}