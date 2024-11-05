mod models;
mod formats;

use lief::pe::headers::MachineType;
use rayon::ThreadPoolBuilder;
use formats::pe::PE;
use models::disassembler::Disassembler;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::process;
use std::fs::File;
use std::io::Write;
use std::collections::HashSet;
use std::io::ErrorKind;
use crate::models::cfg::CFG;
use crate::models::block::Block;
use crate::models::function::Function;
use crate::models::binary::BinaryArchitecture;
use crate::models::config::ARGS;

fn main() {

    let pe = match PE::new(ARGS.input.clone()) {
        Ok(pe) => pe,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let machine = match pe.machine() {
        MachineType::AMD64 => BinaryArchitecture::AMD64,
        MachineType::I386 => BinaryArchitecture::I386,
        _ => BinaryArchitecture::UNKNOWN,
    };

    let disassembler = match Disassembler::new(machine, pe.image(), pe.executable_address_ranges()) {
        Ok(disassembler) => disassembler,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let mut entrypoints = HashSet::<u64>::new();

    if !ARGS.disable_linear_pass {
        entrypoints.extend(disassembler.disassemble_linear_pass(ARGS.linear_pass_jump_threshold, ARGS.linear_pass_instruction_threshold));
    }

    entrypoints.extend(pe.functions());

    let mut cfg = CFG::new();
    cfg.options.enable_sha256 = !ARGS.disable_sha256;
    cfg.options.enable_minhash = !ARGS.disable_minhash;
    cfg.options.enable_tlsh = !ARGS.disable_tlsh;
    cfg.options.minhash_maximum_byte_size = ARGS.minhash_maximum_byte_size;
    cfg.options.minhash_number_of_hashes = ARGS.minhash_number_of_hashes;
    cfg.options.minhash_seed = ARGS.minhash_seed;
    cfg.options.enable_feature = !ARGS.disable_feature;
    cfg.options.enable_normalized = ARGS.enable_normalized;
    cfg.options.tags = ARGS.tags.clone().unwrap_or_default();

    if let Err(error) = disassembler.disassemble_control_flow(entrypoints, &mut cfg) {
        eprintln!("{}", error);
        process::exit(1);
    }

    ThreadPoolBuilder::new()
        .num_threads(ARGS.threads)
        .build_global()
        .expect("failed to build thread pool");

    let blocks: Vec<String> = cfg.blocks.valid()
        .par_iter()
        .filter_map(|address| Block::new(*address, &cfg).ok())
        .filter_map(|block|block.json().ok())
        .collect();

    let functions: Vec<String> = cfg.functions.valid()
        .par_iter()
        .filter_map(|address| Function::new(*address, &cfg).ok())
        .filter_map(|function| function.json().ok())
        .collect();

    if ARGS.output.is_none() {
        functions.iter().for_each(|result| {
            writeln!(std::io::stdout(), "{}", result).unwrap_or_else(|e| {
                if e.kind() == ErrorKind::BrokenPipe {
                    std::process::exit(0);
                } else {
                    eprintln!("error writing to stdout: {}", e);
                    std::process::exit(1);
                }
            });
        });

        blocks.iter().for_each(|result| {
            writeln!(std::io::stdout(), "{}", result).unwrap_or_else(|e| {
                if e.kind() == ErrorKind::BrokenPipe {
                    std::process::exit(0);
                } else {
                    eprintln!("error writing to stdout: {}", e);
                    std::process::exit(1);
                }
            });
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

    // if let Some(output_file) = &ARGS.output {
    //     let mut file = match File::create(output_file) {
    //         Ok(file) => file,
    //         Err(error) => {
    //             eprintln!("{}", error);
    //             std::process::exit(1);
    //         }
    //     };

    //     for block in blocks {
    //         if let Err(error) = writeln!(file, "{}", block) {
    //             eprintln!("{}", error);
    //             std::process::exit(1);
    //         }
    //     }

    // }


    // stop HERE


    // let functions: Vec<String> = cfg.functions()
    //     .par_iter()
    //     .filter_map(|function| function.json().ok())
    //     .collect();

    // let blocks: Vec<String> = cfg.blocks()
    //     .par_iter()
    //     .filter_map(|function| function.json().ok())
    //     .collect();

    // if ARGS.output.is_none() {
    //     functions.iter().for_each(|result| {
    //         writeln!(std::io::stdout(), "{}", result).unwrap_or_else(|e| {
    //             if e.kind() == ErrorKind::BrokenPipe {
    //                 std::process::exit(0);
    //             } else {
    //                 eprintln!("error writing to stdout: {}", e);
    //                 std::process::exit(1);
    //             }
    //         });
    //     });

    //     blocks.iter().for_each(|result| {
    //         writeln!(std::io::stdout(), "{}", result).unwrap_or_else(|e| {
    //             if e.kind() == ErrorKind::BrokenPipe {
    //                 std::process::exit(0);
    //             } else {
    //                 eprintln!("{}", e);
    //                 std::process::exit(1);
    //             }
    //         });
    //     });
    // }

    // if let Some(output_file) = &ARGS.output {
    //     let mut file = match File::create(output_file) {
    //         Ok(file) => file,
    //         Err(error) => {
    //             eprintln!("{}", error);
    //             std::process::exit(1);
    //         }
    //     };

    //     for function in functions {
    //         if let Err(error) = writeln!(file, "{}", function) {
    //             eprintln!("{}", error);
    //             std::process::exit(1);
    //         }
    //     }
    //     for block in blocks {
    //         if let Err(error) = writeln!(file, "{}", block) {
    //             eprintln!("{}", error);
    //             std::process::exit(1);
    //         }
    //     }

    // }

    process::exit(0);

}