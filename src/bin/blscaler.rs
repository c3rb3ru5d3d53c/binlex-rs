use rayon::prelude::*;
use serde_json::Value;
use std::io::{self, BufRead};
use std::process;

fn normalize(data: &[f64]) -> Vec<f64> {
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    data.iter().map(|&x| (x - min) / (max - min)).collect()
}

/// Parses a JSON line and validates it as an array of numbers.
fn parse_line_to_numbers(line: &str) -> Result<Vec<f64>, std::io::Error> {
    let parsed: Value = serde_json::from_str(line)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid JSON format"))?;

    let array = parsed.as_array().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Input is not a JSON array")
    })?;

    array.iter().map(|v| v.as_f64().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Array contains non-numeric values")
    })).collect()
}

/// Processes a single line of input by parsing, normalizing, and serializing it.
fn process_line(line: &str) -> Result<String, std::io::Error> {
    let features = parse_line_to_numbers(line)?;
    let normalized = normalize(&features);
    serde_json::to_string(&normalized).map_err(|_| {
        io::Error::new(io::ErrorKind::Other, "Failed to serialize JSON")
    })
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|line| {
        line.expect("Failed to read line")
    }).collect();

    // Process each line in parallel and collect results, exiting on any error
    let results: Result<Vec<String>, _> = lines
        .par_iter()
        .map(|line| process_line(line))
        .collect();

    match results {
        Ok(output_lines) => {
            for line in output_lines {
                println!("{}", line);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}
