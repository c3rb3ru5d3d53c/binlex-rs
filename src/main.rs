mod models;
mod formats;
mod types;

use rayon::ThreadPoolBuilder;
use formats::pe::PE;
use models::disassemblers::capstone::disassembler::Disassembler;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_json::json;
use std::process;
use std::fs::File;
use std::io::Write;
use std::collections::BTreeSet;
use crate::models::controlflow::graph::Graph;
use crate::models::controlflow::block::Block;
use crate::models::controlflow::function::Function;
use crate::types::lz4string::LZ4String;
use crate::models::terminal::args::ARGS;
use crate::models::terminal::io::Stdout;
use memmap2::Mmap;
use crate::models::terminal::io::JSON;
use crate::models::controlflow::symbol::Symbol;

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

    ThreadPoolBuilder::new()
        .num_threads(ARGS.threads)
        .build_global()
        .expect("failed to build thread pool");

    let pe = match PE::new(ARGS.input.clone()) {
        Ok(pe) => pe,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let function_symbols = get_pe_function_symbols(&pe);

    let machine = pe.machine();

    #[allow(unused_assignments)]
    let mut image_bytes = Vec::<u8>::new();
    let image_mmap: Mmap;
    let image: &[u8];

    if ARGS.enable_file_mapping {
        match pe.imagecache(
            ARGS.file_mapping_directory.clone().unwrap(),
            ARGS.enable_file_mapping_cache) {
            Ok(mapped) => {
                image_mmap = mapped.mmap().unwrap();
                image = &image_mmap;
            }
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            }
        };
    } else {
        image_bytes = pe.image();
        image = &image_bytes;
    }

    let executable_address_ranges = pe.executable_address_ranges();

    let disassembler = match Disassembler::new(machine, &image, executable_address_ranges.clone()) {
        Ok(disassembler) => disassembler,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let mut entrypoints = BTreeSet::<u64>::new();

    if !ARGS.disable_linear_pass {
        entrypoints.extend(disassembler.disassemble_linear_pass(ARGS.linear_pass_jump_threshold, ARGS.linear_pass_instruction_threshold));
    }

    entrypoints.extend(pe.functions());

    let function_symbol_addresses: BTreeSet<u64> = function_symbols
        .iter()
        .map(|symbol| symbol.address)
        .collect();

    entrypoints.extend(function_symbol_addresses);

    let mut cfg = Graph::new(machine);
    cfg.options.enable_sha256 = !ARGS.disable_sha256;
    cfg.options.enable_minhash = !ARGS.disable_minhash;
    cfg.options.enable_tlsh = !ARGS.disable_tlsh;
    cfg.options.minhash_maximum_byte_size = ARGS.minhash_maximum_byte_size;
    cfg.options.minhash_number_of_hashes = ARGS.minhash_number_of_hashes;
    cfg.options.minhash_seed = ARGS.minhash_seed;
    cfg.options.enable_feature = !ARGS.disable_feature;
    cfg.options.enable_normalized = ARGS.enable_normalized;
    cfg.options.tags = ARGS.tags.clone().unwrap_or_default();
    cfg.options.file_sha256 = pe.sha256();
    cfg.options.file_tlsh = pe.tlsh();
    cfg.options.file_size = Some(pe.size());
    cfg.functions.enqueue_extend(entrypoints);
    cfg.functions.insert_symbols_extend(function_symbols);

    while !cfg.functions.queue.is_empty() {
        let function_addresses = cfg.functions.dequeue_all();
        cfg.functions.insert_processed_extend(function_addresses.clone());
        let graphs: Vec<Graph> = function_addresses
            .par_iter()
            .map(|address| {
                let mut graph = Graph::new(machine);
                graph.options = cfg.options.clone();
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

    if ARGS.output.is_none() {
        functions.iter().for_each(|result| {
            Stdout.print(result);
        });

        blocks.iter().for_each(|result| {
            Stdout.print(result);
        });
    }

     if let Some(output_file) = &ARGS.output {
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
