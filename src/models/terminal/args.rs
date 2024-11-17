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
    pub enable_mmap_cache: bool,
    #[arg(long)]
    pub mmap_directory: Option<String>,
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
        if let Err(error) = config.from_default() {
            eprintln!("{}", error);
            process::exit(1);
        }
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
        config.heuristics.features.enabled = !args.disable_features;
    }

    if args.disable_sha256 != false {
        config.hashing.sha256.enabled = !args.disable_sha256;
    }

    if args.disable_entropy != false {
        config.heuristics.entropy.enabled = !args.disable_entropy;
    }

    if args.disable_minhash != false {
        config.hashing.minhash.enabled = !args.disable_minhash;
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

    if args.mmap_directory.is_some() {
        config.mmap.directory = args.mmap_directory.unwrap();
    }

    if args.enable_mmap_cache != false {
        config.mmap.cache.enabled = args.enable_mmap_cache;
    }

    if args.disable_tlsh != false {
        config.hashing.tlsh.enabled = !args.disable_tlsh;
    }

    if args.tlsh_minimum_byte_size.is_some() {
        config.hashing.tlsh.minimum_byte_size = args.tlsh_minimum_byte_size.unwrap();
    }

    if args.enable_normalized != false {
        config.heuristics.normalization.enabled = args.enable_normalized;
    }

    if args.disable_linear_pass != false {
        config.disassembler.sweep.enabled = !args.disable_linear_pass;
    }

    if args.tags.is_some() {
        config.general.tags = args.tags.unwrap();
    }

    if args.disable_hashing == true {
        config.hashing.minhash.enabled = false;
        config.hashing.sha256.enabled = false;
        config.hashing.tlsh.enabled = false;
    }

    if args.minimal == true || config.general.minimal == true {
        config.hashing.minhash.enabled = false;
        config.hashing.sha256.enabled = false;
        config.hashing.tlsh.enabled = false;
        config.heuristics.entropy.enabled = false;
        config.heuristics.features.enabled = false;
        config.heuristics.normalization.enabled = false;
    }

    config

}

pub static CONFIG: Lazy<Config> = Lazy::new(parse);
