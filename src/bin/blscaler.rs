use clap::Parser;
use rayon::prelude::*;
use serde_json::Value;
use std::io::IsTerminal;
use std::io::{self, BufRead};
use serde_json::Number;
use std::fs::File;
use std::io::Write;
use std::process;
use rayon::ThreadPoolBuilder;
use binlex::types::lz4string::LZ4String;
use binlex::models::terminal::args::VERSION;
use binlex::models::terminal::args::AUTHOR;
use binlex::models::terminal::io::Stdout;
use serde_json::de::Deserializer;

#[derive(Parser, Debug)]
#[command(
    name = "blscaler",
    version = VERSION,
    about = format!("A Binlex ML Scaler Tool\n\nVersion: {}", VERSION),
    after_help = format!("Author: {}", AUTHOR),
)]
struct Args {
    #[arg(short, long)]
    input: Option<String>,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(short, long, default_value_t = 1)]
    threads: usize
}

fn normalize(data: &[f64]) -> Vec<f64> {
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    data.iter().map(|&x| (x - min) / (max - min)).collect()
}

fn process_value(mut parsed: Value) -> String {
    if let Some(feature) = parsed
        .get_mut("signature")
        .and_then(|signature| signature.get_mut("feature"))
    {
        if let Some(array) = feature.as_array() {
            let values: Vec<f64> = array.iter().filter_map(|v| v.as_f64()).collect();
            let normalized_values = normalize(&values);
            *feature = Value::Array(
                normalized_values
                    .into_iter()
                    .filter_map(|num| Number::from_f64(num).map(Value::Number)) // Filter out non-finite numbers
                    .collect(),
            );
        }
    }
    let result = match serde_json::to_string(&parsed) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };
    return result;
}

fn main() {

    let args = Args::parse();

    ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .expect("failed to build thread pool");

    let input_reader: Box<dyn BufRead> = if let Some(input) = args.input.clone() {
        let file = match File::open(input) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            },
        };
        Box::new(io::BufReader::new(file))
    } else {
        if io::stdin().is_terminal() {
            eprintln!("failed to read standard input");
            process::exit(1);
        }
        Box::new(io::BufReader::new(io::stdin()))
    };

    let values: Vec<Value> = Deserializer::from_reader(input_reader)
        .into_iter::<Value>()
        .map(|value| match value {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Error parsing JSON: {}", error);
                process::exit(1);
            }
        })
        .collect();

    let results: Vec<LZ4String> = values.into_par_iter()
        .map(|value| {
            LZ4String::from(process_value(value))
        })
        .collect();

    if let Some(output_file) = args.output {
        let mut file = match File::create(output_file) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        };
        for result in results {
            if let Err(error) = writeln!(file, "{}", result) {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }
    } else {
        results.iter().for_each(|result| {
            Stdout.print(result);
        });
    }

    process::exit(0);

}
