
use crate::models::controlflow::instruction::Instruction;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::Error;
use std::io::ErrorKind;
use crate::models::binary::Binary;
use crate::models::controlflow::graph::Graph;
use crate::models::controlflow::graph::GraphQueue;
use crate::models::controlflow::block::Block;
use crate::models::controlflow::signature::Signature;
use crate::models::controlflow::signature::SignatureJson;
use crate::models::controlflow::file::FileJson;
use crate::models::controlflow::file::File;
use crate::models::hashing::sha256::SHA256;
use crate::models::hashing::tlsh::TLSH;
use crate::models::hashing::minhash::MinHash32;

#[derive(Serialize, Deserialize)]
pub struct FunctionQueueJson {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub offset: Option<u64>,
    pub relative_virtual_address: Option<u64>,
    pub virtual_address: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct FunctionJson {
    #[serde(rename = "type")]
    pub type_: String,
    pub address: u64,
    pub edges: usize,
    pub prologue: bool,
    pub signature: Option<SignatureJson>,
    pub size: Option<usize>,
    pub bytes: Option<String>,
    pub functions: BTreeMap<u64, u64>,
    pub blocks: BTreeSet<u64>,
    pub file: Option<FileJson>,
    pub instructions: usize,
    pub entropy: Option<f64>,
    pub sha256: Option<String>,
    pub minhash: Option<String>,
    pub tlsh: Option<String>,
    pub contiguous: bool,
    pub tags: Vec<String>,
}

#[derive(Clone)]
pub struct Function <'function>{
    pub address: u64,
    pub cfg: &'function Graph,
    pub blocks: BTreeMap<u64, Instruction>,
    pub functions: BTreeMap<u64, u64>,
    pub instruction_count: usize,
    pub edges: usize,
    pub is_prologue: bool,
    pub size: usize,
}

impl<'function> Function<'function> {

    pub fn new(address: u64, cfg: &'function Graph) -> Result<Self, Error> {

        if !cfg.functions.is_valid(address) {
            return Err(Error::new(ErrorKind::Other, format!("Function -> 0x{:x}: is not valid", address)));
        }

        let mut blocks = BTreeMap::<u64, Instruction>::new();
        let mut functions = BTreeMap::<u64, u64>::new();
        let mut instruction_count: usize = 0;
        let mut edges: usize = 0;
        let mut is_prologue = false;
        let mut size: usize = 0;

        let mut queue = GraphQueue::new();

        queue.enqueue(address);

        while let Some(block_address) = queue.dequeue() {
            queue.set_processed(block_address);
            if cfg.blocks.is_invalid(block_address) {
                return Err(Error::new(ErrorKind::Other, format!("Function -> 0x{:x} -> Block -> 0x{:x}: is invalid", address, block_address)));
            }
            match Block::new(block_address, &cfg) {
                Ok(block) => {
                    if block.address == address {
                        is_prologue = block.is_prologue();
                    }
                    queue.enqueue_extend(block.blocks());
                    functions.extend(block.functions());
                    size += block.size();
                    instruction_count += block.instruction_count();
                    edges += block.edges();
                    blocks.insert(block_address, block.terminator.clone());
                }
                Err(error) => {
                    return Err(Error::new(
                    ErrorKind::Other,
                    format!("Function -> 0x{:x} -> Block -> 0x{:x}: {}", address, block_address, error)
                ))},
            }
        }

        return Ok(Self {
            address: address,
            cfg: cfg,
            blocks: blocks,
            functions: functions,
            instruction_count: instruction_count,
            edges: edges,
            is_prologue: is_prologue,
            size: size,
        });
    }

    pub fn process(&self) -> FunctionJson {
        FunctionJson {
            address: self.address,
            type_: "function".to_string(),
            edges: self.edges(),
            prologue: self.is_prologue(),
            signature: self.signature(),
            bytes: self.bytes_to_hex(),
            size: self.size(),
            functions: self.functions(),
            blocks: self.block_addresses(),
            instructions: self.instruction_count(),
            entropy: self.entropy(),
            sha256: self.sha256(),
            minhash: self.minhash(),
            tlsh: self.tlsh(),
            contiguous: self.is_contiguous(),
            file: self.file(),
            tags: self.cfg.options.tags.clone(),
        }
    }

    pub fn file(&self) -> Option<FileJson> {
        Some(File::new(self.cfg.options.clone()).process())
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if let Ok(json) = self.json() {
            println!("{}", json);
        }
    }

    pub fn json(&self) -> Result<String, Error> {
        let raw = self.process();
        let result = serde_json::to_string(&raw)?;
        Ok(result)
    }

    pub fn signature(&self) -> Option<SignatureJson> {
        if !self.is_contiguous() { return None; }
        return Some(Signature::new(self.address, self.end().unwrap(), &self.cfg, self.cfg.options.clone()).process());
    }


    pub fn instruction_count(&self) -> usize {
        return self.instruction_count;
    }

    pub fn is_prologue(&self) -> bool {
        return self.is_prologue;
    }

    pub fn block_addresses(&self) -> BTreeSet<u64> {
        self.blocks().keys().cloned().collect()
    }

    pub fn edges(&self) -> usize {
        return self.edges;
    }

    pub fn bytes_to_hex(&self) -> Option<String> {
        if let Some(bytes) = self.bytes() {
            return Some(Binary::to_hex(&bytes));
        }
        return None;
    }

    pub fn size(&self) -> Option<usize> {
        if !self.is_contiguous() { return None; }
        return Some(self.size);
    }

    pub fn end(&self) -> Option<u64> {
        if !self.is_contiguous() { return None; }
        self.blocks().iter().last().map(|(_, terminator)|terminator.address)
    }

    pub fn bytes(&self) -> Option<Vec<u8>> {
        if !self.is_contiguous() { return None; }
        let mut result = Vec::<u8>::new();
        for entry in self.cfg.instructions.range(self.address..=self.end().unwrap()) {
            let instruction = entry.value();
            result.extend(instruction.bytes.clone());
        }
        return Some(result);
    }

    pub fn sha256(&self) -> Option<String> {
        if !self.cfg.options.enable_sha256 { return None; }
        if !self.is_contiguous() { return None; }
        if let Some(bytes) = self.bytes() {
            return SHA256::new(&bytes).hexdigest();
        }
        return None;
    }

    pub fn entropy(&self) -> Option<f64> {
        if !self.cfg.options.enable_entropy { return None; }
        if !self.is_contiguous() { return None; }
        if let Some(bytes) = self.bytes() {
            return Binary::entropy(&bytes);
        }
        return None;
    }

    pub fn tlsh(&self) -> Option<String> {
        if !self.cfg.options.enable_tlsh { return None; }
        if !self.is_contiguous() { return None; }
        if let Some(bytes) = self.bytes() {
            return TLSH::new(&bytes, self.cfg.options.tlsh_mininum_byte_size).hexdigest();
        }
        return None;
    }

    pub fn minhash(&self) -> Option<String> {
        if !self.cfg.options.enable_minhash { return None; }
        if !self.is_contiguous() { return None; }
        if let Some(bytes) = self.bytes() {
            if bytes.len() > self.cfg.options.minhash_maximum_byte_size { return None; }
            return MinHash32::new(
                &bytes,
                self.cfg.options.minhash_number_of_hashes,
                self.cfg.options.minhash_shingle_size,
                self.cfg.options.minhash_seed).hexdigest();
        }
        return None;
    }

    pub fn blocks(&self) -> &BTreeMap<u64, Instruction> {
        return &self.blocks;
    }

    pub fn functions(&self) -> BTreeMap<u64, u64> {
        return self.functions.clone();
    }

    pub fn is_contiguous(&self) -> bool {
        let mut block_previous_end: Option<u64> = None;
        for (block_start_address, terminator )in self.blocks() {
            if let Some(previous_end) = block_previous_end {
                if previous_end != *block_start_address {
                    return false;
                }
            }
            block_previous_end = Some(terminator.address + terminator.size() as u64);
        }
        return true;
    }
}