use std::io::{stdin, ErrorKind};
use std::io::{self, BufRead, BufReader, IsTerminal, Write};
use std::fmt::Display;
use std::process;
use std::fs::File;
use serde_json::{Value, Deserializer};
use std::fmt;

/// Represents a wrapper for standard input operations.
pub struct Stdin;

/// Represents a wrapper for standard output operations.
pub struct Stdout;

/// Represents a wrapper for standard error operations.
pub struct Stderr;

impl Stdin {

    #[allow(dead_code)]
    pub fn is_terminal() -> bool {
        stdin().is_terminal()
    }

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

#[derive(Debug)]
pub enum JSONError {
    FileOpenError(String),
    StdinReadError,
    JSONParseError(String),
    JSONToStringError(String),
    FileWriteError(String),
}

impl fmt::Display for JSONError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JSONError::FileOpenError(path) => write!(f, "failed to open file: {}", path),
            JSONError::StdinReadError => write!(f, "failed to read from standard input"),
            JSONError::JSONParseError(err) => write!(f, "failed parsing json: {}", err),
            JSONError::JSONToStringError(err) => write!(f, "error converting json value to string: {}", err),
            JSONError::FileWriteError(path) => write!(f, "failed to write to file: {}", path),
        }
    }
}

pub struct JSON {
    values: Vec<Value>,
}

impl JSON {
    /// Constructs a `JSON` instance from a file path.
    #[allow(dead_code)]
    pub fn from_file(path: &str) -> Result<Self, JSONError> {
        let file = File::open(path).map_err(|_| JSONError::FileOpenError(path.to_string()))?;
        let reader = BufReader::new(file);
        Self::deserialize(reader)
    }

    /// Constructs a `JSON` instance from standard input.
    #[allow(dead_code)]
    pub fn from_stdin() -> Result<Self, JSONError> {
        if io::stdin().is_terminal() {
            return Err(JSONError::StdinReadError);
        }

        let reader = BufReader::new(io::stdin());
        Self::deserialize(reader)
    }

    /// Constructs a `JSON` instance from a file path or standard input.
    /// If the file path is `None`, reads from standard input.
    #[allow(dead_code)]
    pub fn from_file_or_stdin(path: Option<String>) -> Result<Self, JSONError> {
        match path {
            Some(file_path) => Self::from_file(&file_path),
            None => Self::from_stdin(),
        }
    }

    /// Private method to deserialize JSON from a given reader.
    #[allow(dead_code)]
    fn deserialize<R: BufRead>(reader: R) -> Result<Self, JSONError> {
        let values: Vec<Value> = Deserializer::from_reader(reader)
            .into_iter::<Value>()
            .map(|value| value.map_err(|e| JSONError::JSONParseError(e.to_string())))
            .collect::<Result<_, _>>()?;

        Ok(JSON { values })
    }

    /// Private method to deserialize JSON with filtering and in-place modification.
    #[allow(dead_code)]
    fn deserialize_with_filter<R, F>(reader: R, filter: F) -> Result<Self, JSONError>
    where
        R: BufRead,
        F: Fn(&mut Value) -> bool,
    {
        let mut values = Vec::new();

        for item in Deserializer::from_reader(reader).into_iter::<Value>() {
            match item {
                Ok(mut value) => {
                    if filter(&mut value) {
                        values.push(value);
                    }
                }
                Err(e) => return Err(JSONError::JSONParseError(e.to_string())),
            }
        }

        Ok(JSON { values })
    }

    /// Constructs a `JSON` instance from a file path with filtering and in-place modification.
    #[allow(dead_code)]
    pub fn from_file_with_filter<F>(path: &str, filter: F) -> Result<Self, JSONError>
    where
        F: Fn(&mut Value) -> bool,
    {
        let file = File::open(path).map_err(|_| JSONError::FileOpenError(path.to_string()))?;
        let reader = BufReader::new(file);
        Self::deserialize_with_filter(reader, filter)
    }

    /// Constructs a `JSON` instance from standard input with filtering and in-place modification.
    pub fn from_stdin_with_filter<F>(filter: F) -> Result<Self, JSONError>
    where
        F: Fn(&mut Value) -> bool,
    {
        if io::stdin().is_terminal() {
            return Err(JSONError::StdinReadError);
        }

        let reader = BufReader::new(io::stdin());
        Self::deserialize_with_filter(reader, filter)
    }

    /// Constructs a `JSON` instance from a file path or standard input with filtering and in-place modification.
    #[allow(dead_code)]
    pub fn from_file_or_stdin_with_filter<F>(path: Option<String>, filter: F) -> Result<Self, JSONError>
    where
        F: Fn(&mut Value) -> bool,
    {
        match path {
            Some(file_path) => Self::from_file_with_filter(&file_path, filter),
            None => Self::from_stdin_with_filter(filter),
        }
    }

    /// Returns a reference to the parsed JSON values.
    #[allow(dead_code)]
    pub fn values(&self) -> &Vec<Value> {
        &self.values
    }

    /// Converts a `serde_json::Value` to a `String`.
    #[allow(dead_code)]
    pub fn value_to_string(value: &Value) -> Result<String, JSONError> {
        serde_json::to_string(value).map_err(|e| JSONError::JSONToStringError(e.to_string()))
    }

    /// Converts all `serde_json::Value`s into a `Vec<String>`.
    #[allow(dead_code)]
    pub fn values_as_strings(&self) -> Vec<String> {
        self.values
            .iter()
            .filter_map(|value| Self::value_to_string(value).ok())
            .collect()
    }

    /// Writes all JSON values as single-line strings to a file.
    #[allow(dead_code)]
    pub fn write_to_file(&self, file_path: &str) -> Result<(), JSONError> {
        let strings = self.values_as_strings();

        let mut file = File::create(file_path).map_err(|_| JSONError::FileWriteError(file_path.to_string()))?;

        for line in strings {
            writeln!(file, "{}", line).map_err(|_| JSONError::FileWriteError(file_path.to_string()))?;
        }

        Ok(())
    }
}
