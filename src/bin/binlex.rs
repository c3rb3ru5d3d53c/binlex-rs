use rayon::ThreadPoolBuilder;
use binlex::formats::pe::PE;
use binlex::disassemblers::capstone::Disassembler;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_json::json;
use std::process;
use std::fs::File;
use std::io::Write;
use std::collections::BTreeSet;
use std::collections::HashSet;
use binlex::controlflow::Graph;
use binlex::controlflow::Block;
use binlex::controlflow::Function;
use binlex::types::LZ4String;
use binlex::terminal::io::Stdout;
use binlex::terminal::io::JSON;
use binlex::controlflow::Symbol;
use clap::Parser;
use binlex::config::Config;
use binlex::config::VERSION;
use binlex::config::AUTHOR;

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
    pub disable_file_hashing: bool,
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

fn validate_args(args: &Args) {

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

fn get_config() -> Config {

    let args = Args::parse();

    validate_args(&args);

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

fn get_pe_function_symbols(pe: &PE) -> Vec<Symbol> {
    let mut symbols = Vec::<Symbol>::new();

    let json = JSON::from_stdin_with_filter(|value| {
        let obj = match value.as_object_mut() {
            Some(obj) => obj,
            None => return false,
        };

        let obj_type = obj.get("type").and_then(|v| v.as_str()).map(String::from);
        let file_offset = obj.get("file_offset").and_then(|v| v.as_u64());
        let relative_virtual_address = obj.get("relative_virtual_address").and_then(|v| v.as_u64());
        let mut virtual_address = obj.get("virtual_address").and_then(|v| v.as_u64());

        if obj_type.as_deref() != Some("function") {
            return false;
        }

        if file_offset.is_none() && relative_virtual_address.is_none() && virtual_address.is_none() {
            return false;
        }

        if virtual_address.is_some() {
            return true;
        }

        if virtual_address.is_none() {
            if let Some(rva) = relative_virtual_address {
                virtual_address = Some(pe.relative_virtual_address_to_virtual_address(rva));
            }
            if let Some(offset) = file_offset {
                if let Some(va) = pe.file_offset_to_virtual_address(offset) {
                    virtual_address = Some(va);
                }
            }

            if let Some(va) = virtual_address {
                obj.insert("virtual_address".to_string(), json!(va));
                return true;
            }
        }

        false

    });

    if json.is_ok() {
        for value in json.unwrap().values() {
            let address = value.get("virtual_address").and_then(|v| v.as_u64());
            if address.is_some() {
                let mut symbol = Symbol::new(address.unwrap());
                if let Some(names) = value.get("names").and_then(|v| v.as_array()) {
                    for name in names {
                        if let Some(name_str) = name.as_str() {
                            symbol.insert_name(name_str.to_string());
                        }
                    }
                }
                symbols.push(symbol);
            }
        }
    }

    return symbols;
}

fn main() {

    let config = get_config();

    ThreadPoolBuilder::new()
        .num_threads(config.general.threads)
        .build_global()
        .expect("failed to build thread pool");

    let pe = match PE::new(config.general.input.clone().unwrap()) {
        Ok(pe) => pe,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let function_symbols = get_pe_function_symbols(&pe);

    let machine = pe.architecture();

    let image = pe.image(config.mmap.directory.clone(), config.mmap.cache.enabled)
        .unwrap_or_else(|error| { eprintln!("{}", error); process::exit(1)})
        .mmap()
        .unwrap_or_else(|error| { eprintln!("{}", error); process::exit(1); });

    let executable_address_ranges = pe.executable_virtual_address_ranges();

    let disassembler = match Disassembler::new(machine, &image, executable_address_ranges.clone()) {
        Ok(disassembler) => disassembler,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let mut entrypoints = BTreeSet::<u64>::new();

    entrypoints.extend(pe.functions());

    if config.disassembler.sweep.enabled {
        entrypoints.extend(disassembler.disassemble_sweep());
    }

    let function_symbol_addresses: BTreeSet<u64> = function_symbols
        .iter()
        .map(|symbol| symbol.address)
        .collect();

    entrypoints.extend(function_symbol_addresses);

    let mut cfg = Graph::new(machine, &config);

    cfg.functions.enqueue_extend(entrypoints);
    cfg.functions.insert_symbols_extend(function_symbols);

    while !cfg.functions.queue.is_empty() {
        let function_addresses = cfg.functions.dequeue_all();
        cfg.functions.insert_processed_extend(function_addresses.clone());
        let graphs: Vec<Graph> = function_addresses
            .par_iter()
            .map(|address| {
                let mut graph = Graph::new(machine, &config);
                //graph.options = cfg.options.clone();
                if let Ok(disasm) = Disassembler::new(machine, &image, executable_address_ranges.clone()) {
                    let _ = disasm.disassemble_function(*address, &mut graph);
                }
                graph
            })
            .collect();
        for mut graph in graphs {
            cfg.absorb(&mut graph);
        }
    }

    let cfg = cfg;

    let blocks: Vec<LZ4String> = cfg.blocks.valid()
        .iter()
        .map(|entry| *entry)
        .collect::<Vec<u64>>()
        .par_iter()
        .filter_map(|address| Block::new(*address, &cfg).ok())
        .filter_map(|block|block.json().ok())
        .map(|js| LZ4String::new(&js))
        .collect();

    let functions: Vec<LZ4String> = cfg.functions.valid()
        .iter()
        .map(|entry| *entry)
        .collect::<Vec<u64>>()
        .par_iter()
        .filter_map(|address| Function::new(*address, &cfg).ok())
        .filter_map(|function| function.json().ok())
        .map(|js| LZ4String::new(&js))
        .collect();

    if config.general.output.is_none() {
        functions.iter().for_each(|result| {
            Stdout.print(result);
        });

        blocks.iter().for_each(|result| {
            Stdout.print(result);
        });
    }

     if let Some(output_file) = &config.general.output {
        let mut file = match File::create(output_file) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        };

        for function in functions {
            if let Err(error) = writeln!(file, "{}", function) {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }
        for block in blocks {
            if let Err(error) = writeln!(file, "{}", block) {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }
    }

    process::exit(0);

}
