use rayon::ThreadPoolBuilder;
use binlex::formats::pe::PE;
use binlex::models::disassemblers::capstone::disassembler::Disassembler;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_json::json;
use std::process;
use std::fs::File;
use std::io::Write;
use std::collections::BTreeSet;
use binlex::models::controlflow::graph::Graph;
use binlex::models::controlflow::block::Block;
use binlex::models::controlflow::function::Function;
use binlex::types::lz4string::LZ4String;
use binlex::models::terminal::args::CONFIG;
use binlex::models::terminal::io::Stdout;
use memmap2::Mmap;
use binlex::models::terminal::io::JSON;
use binlex::models::controlflow::symbol::Symbol;

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
        .num_threads(CONFIG.general.threads)
        .build_global()
        .expect("failed to build thread pool");

    let pe = match PE::new(CONFIG.general.input.clone().unwrap()) {
        Ok(pe) => pe,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let function_symbols = get_pe_function_symbols(&pe);

    let machine = pe.architecture();

    let image_mmap: Mmap;
    let image: &[u8];

    match pe.image(
        CONFIG.mmap.directory.clone(),
        CONFIG.mmap.cache.enabled) {
        Ok(mapped) => {
            image_mmap = mapped.mmap().unwrap();
            image = &image_mmap;
        }
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

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

    if CONFIG.disassembler.sweep.enabled {
        entrypoints.extend(disassembler.disassemble_sweep());
    }

    let function_symbol_addresses: BTreeSet<u64> = function_symbols
        .iter()
        .map(|symbol| symbol.address)
        .collect();

    entrypoints.extend(function_symbol_addresses);

    let mut cfg = Graph::new(machine, &CONFIG);

    cfg.functions.enqueue_extend(entrypoints);
    cfg.functions.insert_symbols_extend(function_symbols);

    while !cfg.functions.queue.is_empty() {
        let function_addresses = cfg.functions.dequeue_all();
        cfg.functions.insert_processed_extend(function_addresses.clone());
        let graphs: Vec<Graph> = function_addresses
            .par_iter()
            .map(|address| {
                let mut graph = Graph::new(machine, &CONFIG);
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

    if CONFIG.general.output.is_none() {
        functions.iter().for_each(|result| {
            Stdout.print(result);
        });

        blocks.iter().for_each(|result| {
            Stdout.print(result);
        });
    }

     if let Some(output_file) = &CONFIG.general.output {
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
