use std::process;
use std::collections::HashSet;
use clap::Parser;
use once_cell::sync::Lazy;
use crate::models::terminal::config::Config;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = "@c3rb3ru5d3d53c";

#[derive(Parser, Debug)]
#[command(
    name = "binlex",
    version = VERSION,
    about = format!("A Binary Pattern Lexer\n\nVersion: {}", VERSION),
    after_help = format!("Author: {}", AUTHOR),
)]
pub struct Args {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long)]
    pub config: Option<String>,
    #[arg(short, long)]
    pub threads: Option<usize>,
    #[arg(long, value_delimiter = ',', default_value = None)]
    pub tags: Option<Vec<String>>,
    #[arg(long, default_value_t = false)]
    pub minimal: bool,
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
    #[arg(long)]
    pub minhash_seed: Option<u64>,
    #[arg(long)]
    pub minhash_number_of_hashes: Option<usize>,
    #[arg(long)]
    pub minhash_shingle_size: Option<usize>,
    #[arg(long)]
    pub minhash_maximum_byte_size: Option<usize>,
    #[arg(long)]
    pub tlsh_minimum_byte_size: Option<usize>,
    #[arg(long, default_value_t = false)]
    pub disable_linear_pass: bool,
    #[arg(long, default_value_t = false)]
    pub disable_tlsh: bool,
    #[arg(long, default_value_t = false)]
    pub disable_minhash: bool,
    #[arg(long, default_value_t = false)]
    pub disable_sha256: bool,
    #[arg(long, default_value_t = false)]
    pub disable_entropy: bool,
    #[arg(long, default_value_t = false)]
    pub disable_features: bool,
    #[arg(long, default_value_t = false)]
    pub disable_hashing: bool,
    #[arg(long, default_value_t = false)]
    pub enable_normalized: bool,
    #[arg(long, default_value_t = false)]
    pub enable_file_mapping: bool,
    #[arg(long, default_value_t = false)]
    pub enable_file_mapping_cache: bool,
    #[arg(long)]
    pub file_mapping_directory: Option<String>,
}

fn validate(args: &Args) {

    if args.minhash_number_of_hashes.is_some() {
        if args.minhash_number_of_hashes.unwrap() < 16 || args.minhash_number_of_hashes.unwrap() > 128 {
            eprintln!("minhash number of hashes can only be between 16 and 128");
            process::exit(1);
        }
    }
    if args.minhash_shingle_size.is_some() {
        if args.minhash_shingle_size.unwrap() == 0 {
            eprintln!("minhash shingle size must be greater than zero");
            process::exit(1);
        }
    }

    if let Some(tags) = &args.tags {
        let mut unique_tags = HashSet::new();
        for tag in tags {
            if !unique_tags.insert(tag) {
                eprintln!("tags must be unique");
                process::exit(1);
            }
        }
    }

}

fn parse() -> Config {

    let args = Args::parse();

    validate(&args);

    let mut config = Config::new();

    let _ = config.write_default();

    if args.config.is_some() {
        match Config::from_file(&args.config.unwrap().to_string()) {
            Ok(result) => {
                config = result;
            },
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            }
        }
    } else {
        let _ = config.from_default();
    }

    config.general.input = Some(args.input);
    config.general.output = args.output;

    if args.debug != false {
        config.general.debug = args.debug;
    }

    if args.threads.is_some() {
        config.general.threads = args.threads.unwrap();
    }

    if args.disable_features != false {
        config.heuristics.features = !args.disable_features;
    }

    if args.disable_sha256 != false {
        config.hashing.sha256.enable = !args.disable_sha256;
    }

    if args.disable_entropy != false {
        config.heuristics.entropy = !args.disable_entropy;
    }

    if args.disable_minhash != false {
        config.hashing.minhash.enable = !args.disable_minhash;
    }

    if args.minhash_maximum_byte_size.is_some() {
        config.hashing.minhash.maximum_byte_size = args.minhash_maximum_byte_size.unwrap();
    }

    if args.minhash_number_of_hashes.is_some() {
        config.hashing.minhash.number_of_hashes = args.minhash_number_of_hashes.unwrap();
    }

    if args.minhash_shingle_size.is_some() {
        config.hashing.minhash.shingle_size = args.minhash_shingle_size.unwrap();
    }

    if args.minhash_seed.is_some() {
        config.hashing.minhash.seed = args.minhash_seed.unwrap();
    }

    if args.file_mapping_directory.is_some() {
        config.file_mapping.directory = args.file_mapping_directory.unwrap();
    }

    if args.enable_file_mapping != false {
        config.file_mapping.enable = args.enable_file_mapping;
    }

    if args.enable_file_mapping_cache != false {
        config.file_mapping.caching = args.enable_file_mapping_cache;
    }

    if args.disable_tlsh != false {
        config.hashing.tlsh.enable = !args.disable_tlsh;
    }

    if args.tlsh_minimum_byte_size.is_some() {
        config.hashing.tlsh.minimum_byte_size = args.tlsh_minimum_byte_size.unwrap();
    }

    if args.enable_normalized != false {
        config.heuristics.normalization = args.enable_normalized;
    }

    if args.disable_linear_pass != false {
        config.disassembler.sweep = !args.disable_linear_pass;
    }

    if args.tags.is_some() {
        config.general.tags = args.tags.unwrap();
    }

    if args.disable_hashing == true {
        config.hashing.minhash.enable = false;
        config.hashing.sha256.enable = false;
        config.hashing.tlsh.enable = false;
    }

    if args.minimal == true || config.general.minimal == true {
        config.hashing.minhash.enable = false;
        config.hashing.sha256.enable = false;
        config.hashing.tlsh.enable = false;
        config.heuristics.entropy = false;
        config.heuristics.features = false;
        config.heuristics.normalization = false;
    }

    config

}

pub static CONFIG: Lazy<Config> = Lazy::new(parse);
