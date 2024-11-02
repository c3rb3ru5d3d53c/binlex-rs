use std::process;
use std::collections::HashSet;
use clap::Parser;
use once_cell::sync::Lazy;

pub const VERSION: &str = "1.0.0";
pub const AUTHOR: &str = "@c3rb3ru5d3d53c";

#[derive(Parser, Debug)]
#[command(
    name = "binlex",
    version = VERSION,
    about = "A Binlex Binary Pattern Lexer",
    author = AUTHOR,
)]
pub struct Args {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long, default_value_t = 1)]
    pub threads: usize,
    #[arg(long, value_delimiter = ',', default_value = None)]
    pub tags: Option<Vec<String>>,
    #[arg(long, default_value_t = false)]
    pub minimal: bool,
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
    #[arg(long, default_value_t = 2)]
    pub linear_pass_jump_threshold: usize,
    #[arg(long, default_value_t = 4)]
    pub linear_pass_instruction_threshold: usize,
    #[arg(long, default_value_t = 0)]
    pub minhash_seed: u64,
    #[arg(long, default_value_t = 64)]
    pub minhash_number_of_hashes: usize,
    #[arg(long, default_value_t = 4)]
    pub minhash_shingle_size: usize,
    #[arg(long, default_value_t = 50)]
    pub minhash_maximum_byte_size: usize,
    #[arg(long, default_value_t = 50)]
    pub tlsh_minimum_byte_size: usize,
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
    pub disable_feature: bool,
    #[arg(long, default_value_t = false)]
    pub disable_hashing: bool,
}

fn validate(args: &mut Args) {
    if args.minhash_number_of_hashes < 16 || args.minhash_number_of_hashes > 128 {
        eprintln!("minhash number of hashes can only be between 16 and 128");
        process::exit(1);
    }
    if args.minhash_shingle_size == 0 {
        eprintln!("minhash shingle size must be greater than zero");
        process::exit(1);
    }
    if args.linear_pass_instruction_threshold <= 0 {
        eprintln!("linear instruction threshold must be greater than 0");
        process::exit(1);
    }
    if args.linear_pass_jump_threshold <= 0 {
        eprintln!("linear jump threshold must be greater than 0");
        process::exit(1);
    }
    if args.disable_hashing {
        args.disable_minhash = true;
        args.disable_sha256 = true;
        args.disable_tlsh = true;
    }

    if args.minimal {
        args.disable_entropy = true;
        args.disable_sha256 = true;
        args.disable_minhash = true;
        args.disable_tlsh = true;
        args.disable_feature = true;
        args.disable_entropy = true;
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

fn parse() -> Args {
    let mut args = Args::parse();
    validate(&mut args);
    args
}

pub static ARGS: Lazy<Args> = Lazy::new(parse);
