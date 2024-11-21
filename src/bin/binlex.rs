use rayon::ThreadPoolBuilder;
use binlex::formats::pe::PE;
use binlex::disassemblers::capstone::Disassembler;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_json::json;
use std::collections::BTreeMap;
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
use binlex::controlflow::Attributes;
use binlex::controlflow::Tag;

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
    #[arg(long, default_value_t = false)]
    pub disable_hashing: bool,
    #[arg(long, default_value_t = false)]
    pub disable_disassembler_sweep: bool,
    #[arg(long, default_value_t = false)]
    pub disable_heuristics: bool,
    #[arg(long, default_value_t = false)]
    pub enable_mmap_cache: bool,
    #[arg(long)]
    pub mmap_directory: Option<String>,
}

fn validate_args(args: &Args) {

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

fn get_pe_function_symbols(pe: &PE) -> BTreeMap<u64, Symbol> {
    let mut symbols = BTreeMap::<u64, Symbol>::new();

    let json = JSON::from_stdin_with_filter(|value| {
        let obj = match value.as_object_mut() {
            Some(obj) => obj,
            None => return false,
        };

        let obj_type = obj.get("type").and_then(|v| v.as_str()).map(String::from);
        let symbol_type = obj.get("symbol_type").and_then(|v| v.as_str()).map(String::from);
        let file_offset = obj.get("file_offset").and_then(|v| v.as_u64());
        let relative_virtual_address = obj.get("relative_virtual_address").and_then(|v| v.as_u64());
        let mut virtual_address = obj.get("virtual_address").and_then(|v| v.as_u64());

        if obj_type.as_deref() != Some("symbol") {
            return false;
        }

        if symbol_type.is_none() {
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
            let name = value.get("name").and_then(|v| v.as_str());
            let symbol_type = value.get("symbol_type").and_then(|v| v.as_str());
            if address.is_none() { continue; }
            if name.is_none() { continue; }
            if symbol_type.is_none() { continue; }
            let symbol = Symbol::new(
                address.unwrap(),
                symbol_type.unwrap().to_string(),
                name.unwrap().to_string());
            symbols.insert(address.unwrap(),symbol);
        }
    }

    return symbols;
}

fn main() {

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

    if args.debug != false {
        config.general.debug = args.debug;
    }

    if args.threads.is_some() {
        config.general.threads = args.threads.unwrap();
    }

    if args.disable_heuristics == true {
        config.disable_heuristics();
    }

    if args.disable_hashing == true {
        config.disable_hashing();
    }

    if args.mmap_directory.is_some() {
        config.mmap.directory = args.mmap_directory.unwrap();
    }

    if args.enable_mmap_cache != false {
        config.mmap.cache.enabled = args.enable_mmap_cache;
    }

    if args.disable_disassembler_sweep == true {
        config.disassembler.sweep.enabled = false;
    }

    if args.minimal == true || config.general.minimal == true {
        config.enable_minimal();
    }

    ThreadPoolBuilder::new()
        .num_threads(config.general.threads)
        .build_global()
        .expect("failed to build thread pool");

    let pe = match PE::new(args.input, config.clone()) {
        Ok(pe) => pe,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let mut attributes = Attributes::new();

    if !config.general.minimal {
        let file_attribute = pe.file.attribute();
        if args.tags.is_some() {
            for tag in args.tags.unwrap() {
                attributes.push(Tag::new(tag).attribute());
            }
        }
        attributes.push(file_attribute);
    }

    let function_symbols = get_pe_function_symbols(&pe);

    let machine = pe.architecture();

    let mapped_file = pe.image()
        .unwrap_or_else(|error| { eprintln!("{}", error); process::exit(1)});

    let image = mapped_file
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

    entrypoints.extend(function_symbols.keys());

    let mut cfg = Graph::new(machine, config.clone());

    disassembler.disassemble_controlflow(entrypoints, &mut cfg)
        .unwrap_or_else(|error| {
            eprintln!("{}", error);
            process::exit(1);
        });

    let cfg = cfg;

    let blocks: Vec<LZ4String> = cfg.blocks.valid()
        .iter()
        .map(|entry| *entry)
        .collect::<Vec<u64>>()
        .par_iter()
        .filter_map(|address| Block::new(*address, &cfg).ok())
        .filter_map(|block| block.json_with_attributes(attributes.clone()).ok())
        .map(|js| LZ4String::new(&js))
        .collect();

    let functions: Vec<LZ4String> = cfg.functions.valid()
        .iter()
        .map(|entry| *entry)
        .collect::<Vec<u64>>()
        .par_iter()
        .filter_map(|address| Function::new(*address, &cfg).ok())
        .filter_map(|function| {
            let mut function_attributes = attributes.clone();
            let symbol= function_symbols.get(&function.address);
            if symbol.is_some() {
                function_attributes.push(symbol.unwrap().attribute());
            }
            function.json_with_attributes(function_attributes).ok()
        })
        .map(|js| LZ4String::new(&js))
        .collect();

    if args.output.is_none() {
        functions.iter().for_each(|result| {
            Stdout.print(result);
        });

        blocks.iter().for_each(|result| {
            Stdout.print(result);
        });
    }

     if let Some(output_file) = args.output {
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
