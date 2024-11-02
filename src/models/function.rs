use std::io::Error;
use std::collections::{HashSet, HashMap, BTreeMap};
use serde::{Deserialize, Serialize};
use serde_json;
use crate::models::block::Block;
use crate::models::signature::{Signature, SignatureJson};
use crate::models::instruction::Instruction;
use crate::models::binary::Binary;
use crate::models::disassembler::DisassemblerOptions;

pub struct Function {
    pub address: u64,
    pub blocks: BTreeMap<u64, Block>,
    pub options: DisassemblerOptions,
}

#[derive(Serialize, Deserialize)]
pub struct FunctionJson {
    #[serde(rename = "type")]
    pub type_: String,
    pub address: u64,
    pub edges: usize,
    pub prologue: bool,
    pub signature: Option<SignatureJson>,
    pub size: usize,
    pub bytes: Option<String>,
    pub functions: BTreeMap<u64, u64>,
    pub blocks: HashSet<u64>,
    pub instructions: usize,
    pub entropy: Option<f64>,
    pub sha256: Option<String>,
    pub minhash: Option<String>,
    pub tlsh: Option<String>,
    pub contiguous: bool,
    pub tags: Vec<String>,
}

impl Function {
    #[allow(dead_code)]
    pub fn new(address: u64, options: DisassemblerOptions) -> Result<Self, Error> {
        return Ok(Self{
            address: address,
            blocks: BTreeMap::<u64, Block>::new(),
            options: options,
        });
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.blocks().iter().map(|block| block.size()).sum()
    }

    #[allow(dead_code)]
    pub fn blocks(&self) -> Vec<&Block> {
        self.blocks.values().collect()
    }

    pub fn process(&self) -> FunctionJson {
        FunctionJson {
            type_: "function".to_string(),
            address: self.address,
            edges: self.edges(),
            prologue: self.prologue(),
            signature: self.signature(),
            size: self.size(),
            bytes: self.to_hex(),
            functions: self.functions().clone(),
            blocks: self.blocks().iter().map(|block| block.address).collect::<HashSet<_>>(),
            instructions: self.instructions().len(),
            entropy: self.entropy(),
            sha256: self.sha256(),
            minhash: self.minhash(),
            tlsh: self.tlsh(),
            contiguous: self.is_contiguous(),
            tags: self.options.tags.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn json(&self) -> Result<String, Error> {
        let raw = self.process();
        let result = serde_json::to_string(&raw)?;
        Ok(result)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if let Ok(json) = self.json() {
            println!("{}", json);
        }
    }

    #[allow(dead_code)]
    pub fn instructions(&self) -> Vec<&Instruction> {
        self.blocks().iter().flat_map(|block| block.instructions()).collect()
    }

    #[allow(dead_code)]
    pub fn signature(&self) -> Option<SignatureJson> {
        if !self.is_contiguous() { return None; }
        Some(Signature::new(&self.instructions(), self.options.clone()).process())
    }

    #[allow(dead_code)]
    pub fn prologue(&self) -> bool {
        self.blocks()
            .first()
            .map(|block| block.prologue)
            .unwrap_or(false)
    }

    #[allow(dead_code)]
    pub fn edges(&self) -> usize {
        self.blocks().iter().map(|block| block.edges).sum()
    }

    #[allow(dead_code)]
    pub fn bytes(&self) -> Vec<u8> {
        if !self.is_contiguous() { return Vec::<u8>::new() }
        self.blocks().iter().flat_map(|block|block.bytes()).collect()
    }

    #[allow(dead_code)]
    pub fn to_hex(&self) -> Option<String> {
        if !self.is_contiguous() { return None; }
        Some(Binary::to_hex(&self.bytes()))
    }

    #[allow(dead_code)]
    pub fn sha256(&self) -> Option<String> {
        if !self.options.enable_sha256 { return None; }
        if !self.is_contiguous() { return None; }
        Binary::sha256(&self.bytes())
    }

    #[allow(dead_code)]
    pub fn minhash(&self) -> Option<String> {
        if !self.options.enable_minhash { return None; }
        Binary::minhash(
            self.options.minhash_maximum_byte_size,
            self.options.minhash_number_of_hashes,
            self.options.minhash_shingle_size,
            self.options.minhash_seed,
            &self.bytes())
    }

    #[allow(dead_code)]
    pub fn tlsh(&self) -> Option<String> {
        if !self.options.enable_tlsh { return None; }
        if !self.is_contiguous() { return None; }
        Binary::tlsh(&self.bytes(), self.options.tlsh_mininum_byte_size)
    }

    #[allow(dead_code)]
    pub fn entropy(&self) -> Option<f64> {
        if !self.options.enable_entropy { return None; }
        let entropi: Vec<f64> = self.blocks()
            .iter()
            .filter_map(|block| block.entropy())
            .collect();
        if entropi.is_empty() { return None; }
        Some(entropi.iter().sum::<f64>() / entropi.len() as f64)
    }

    #[allow(dead_code)]
    pub fn functions(&self) -> BTreeMap<u64, u64> {
        let mut result = BTreeMap::<u64, u64>::new();
        for block in self.blocks() {
            for (instruction_address, function_address) in block.functions() {
                result.insert(*instruction_address, *function_address);
            }
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn patch_block_overlaps(&mut self) {
        loop {
            let overlaps = self.get_overlapped_block_addresses();

            if overlaps.is_empty() {
                break;
            }

            for (block_start, overlap_address) in overlaps {
                if let Some((_, block)) = self.blocks.iter_mut().find(|(_, b)| b.address == block_start) {
                    let mut instructions_to_remove = HashSet::<u64>::new();
                    for (&instruction_address, _) in &block.instructions {
                        if instruction_address >= overlap_address {
                            instructions_to_remove.insert(instruction_address);
                        }
                    }
                    for address in instructions_to_remove {
                        block.instructions.remove(&address);
                    }

                    let mut functions_to_remove = HashSet::<u64>::new();
                    for (&function_address, _) in &block.functions {
                        if function_address >= overlap_address {
                            functions_to_remove.insert(function_address);
                        }
                    }

                    for address in functions_to_remove {
                        block.functions.remove(&address);
                    }

                    block.next = Some(overlap_address);
                    block.to.clear();
                    block.edges = 0;
                    block.conditional = false;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_overlapped_block_addresses(&self) -> HashMap<u64, u64> {
        let mut overlaps = HashMap::<u64, u64>::new();

        let mut intervals: Vec<(u64, u64)> = self.blocks.iter()
            .map(|(_, block)| {
                (block.address, block.address + block.size() as u64)
            })
            .collect();

        intervals.sort_by_key(|&(start, _)| start);

        for i in 0..intervals.len(){
            let (block_start_0, block_end_0) = intervals[i];
            for j in (i + 1)..intervals.len() {
                let block_start_1= intervals[j].0;
                if block_start_1 >= block_end_0 {
                    break;
                }
                overlaps.insert(block_start_0, block_start_1);
            }
        }
        return overlaps;
    }

    #[allow(dead_code)]
    pub fn has_return(&self) -> bool {
        self.blocks().iter().any(|block| block.has_return())
    }

    #[allow(dead_code)]
    pub fn is_contiguous(&self) -> bool {
        if !self.has_return() { return false };
        let mut previous_end_address: Option<u64> = None;
        for block in self.blocks() {
            let start_address = block.address;
            let end_address = start_address + block.size() as u64;

            if let Some(prev_end) = previous_end_address {
                if start_address != prev_end {
                    return false;
                }
            }
            previous_end_address = Some(end_address);
        }
        true
    }

    #[allow(dead_code)]
    pub fn print_blocks(&self) {
        for block in self.blocks() {
            block.print();
        }
    }

}