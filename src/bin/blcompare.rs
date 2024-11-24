use std::process;
use clap::Parser;
use binlex::AUTHOR;
use binlex::VERSION;
use binlex::io::JSON;
use binlex::io::Stdout;
use binlex::hashing::TLSH;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

#[derive(Serialize, Deserialize)]
pub struct ComparisonJson {
    /// The type of this entity, always `"comparison"`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The starting address of the block.
    pub lhs: Value,
    /// The address of the next sequential block, if any.
    pub rhs: Value,
    /// TLSH Similarity Score
    pub tlsh: Option<u32>,
}

#[derive(Parser, Debug)]
#[command(
    name = "blcompare",
    version = VERSION,
    about =  format!("A Binlex Trait Comparison Tool\n\nVersion: {}", VERSION),
    after_help = format!("Author: {}", AUTHOR),
)]
struct Args {
    #[arg(short, long)]
    input_lhs: Option<String>,
    #[arg(short, long)]
    input_rhs: String,
    #[arg(short, long, default_value_t = 1)]
    pub threads: usize,
}

fn main () {
    let args = Args::parse();

    ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap_or_else(|error| {
            eprintln!("{}", error);
            process::exit(1);
        });

    let json_lhs = JSON::from_file_or_stdin_with_filter(args.input_lhs, |value| {
        let architecture = value.get("architecture").and_then(|v| v.as_str()).map(String::from);
        let tlsh_normalized = value
            .get("signature")
            .and_then(|v| v.get("tlsh"))
            .and_then(|v| v.as_str())
            .map(String::from);
        if tlsh_normalized.is_none() { return false; }
        if architecture.is_none() { return false; }
        true
    }).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    let json_rhs = JSON::from_file_with_filter(&args.input_rhs, |value| {
        let architecture = value.get("architecture").and_then(|v| v.as_str()).map(String::from);
        let tlsh_normalized = value
            .get("signature")
            .and_then(|v| v.get("tlsh"))
            .and_then(|v| v.as_str())
            .map(String::from);
        if tlsh_normalized.is_none() { return false; }
        if architecture.is_none() { return false; }
        true
    }).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    let rhs_entries: Vec<Value> = json_rhs.values().into_iter().cloned().collect();

    json_lhs.values().par_iter().for_each(|value_lhs| {
        let type_lhs = value_lhs.get("type").and_then(|v| v.as_str()).map(String::from).unwrap();
        let architecture_lhs = value_lhs.get("architecture").and_then(|v| v.as_str()).map(String::from).unwrap();
        let tlsh_lhs = value_lhs
            .get("signature")
            .and_then(|v| v.get("tlsh"))
            .and_then(|v| v.as_str())
            .map(String::from).unwrap();

        for value_rhs in &rhs_entries {
            let type_rhs = value_rhs.get("type").and_then(|v| v.as_str()).map(String::from).unwrap();
            let architecture_rhs = value_rhs.get("architecture").and_then(|v| v.as_str()).map(String::from).unwrap();
            let tlsh_rhs = value_rhs
                .get("signature")
                .and_then(|v| v.get("tlsh"))
                .and_then(|v| v.as_str())
                .map(String::from).unwrap();

            if architecture_lhs != architecture_rhs { continue; }
            if type_lhs != type_rhs { continue; }

            let tlsh_similarity = TLSH::compare(tlsh_lhs.clone(), tlsh_rhs.clone()).ok();

            let comparison = ComparisonJson {
                type_: "comparison".to_string(),
                lhs: value_lhs.clone(),
                rhs: value_rhs.clone(),
                tlsh: tlsh_similarity,
            };

            let serialized = match serde_json::to_string(&comparison) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Serialization error: {}", e);
                    continue;
                }
            };

            Stdout::print(serialized);
        }
    });

    process::exit(0);
}
