use crate::models::controlflow::instruction::Instruction;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::Error;
use std::io::ErrorKind;
use crate::models::binary::Binary;
use crate::models::controlflow::graph::Graph;
use crate::models::controlflow::signature::Signature;
use crate::models::controlflow::signature::SignatureJson;
use crate::models::controlflow::file::File;
use crate::models::controlflow::file::FileJson;
use crate::models::hashing::sha256::SHA256;
use crate::models::hashing::tlsh::TLSH;
use crate::models::hashing::minhash::MinHash32;

#[derive(Serialize, Deserialize)]
pub struct BlockJson {
    #[serde(rename = "type")]
    pub type_: String,
    pub address: u64,
    pub next: Option<u64>,
    pub to: BTreeSet<u64>,
    pub edges: usize,
    pub prologue: bool,
    pub conditional: bool,
    pub signature: SignatureJson,
    pub size: usize,
    pub bytes: String,
    pub functions: BTreeMap<u64, u64>,
    pub instructions: usize,
    pub entropy: Option<f64>,
    pub sha256: Option<String>,
    pub minhash: Option<String>,
    pub tlsh: Option<String>,
    pub contiguous: bool,
    pub file: Option<FileJson>,
    pub tags: Vec<String>,
}

#[derive(Clone)]
pub struct Block <'block>{
    pub address: u64,
    pub cfg: &'block Graph,
    pub terminator: Instruction,
}

impl<'block> Block<'block> {

    pub fn new(address: u64, cfg: &'block Graph) -> Result<Self, Error> {

        if !cfg.blocks.is_valid(address) {
            return Err(Error::new(ErrorKind::Other, format!("Block -> 0x{:x}: is not valid", address)));
        }

        let mut terminator: Option<Instruction> = None;

        let previous_address: Option<u64> = None;
        for entry in cfg.instructions.range(address..){
            let instruction = entry.value();
            if let Some(prev_addr) = previous_address{
                if instruction.address != prev_addr {
                    return Err(Error::new(ErrorKind::Other, format!("Block -> 0x{:x}: is not contiguous", address)));
                }
            }
            if instruction.is_jump
                || instruction.is_trap
                || instruction.is_return
                || (address != instruction.address && instruction.is_block_start) {
                terminator = Some(instruction.clone());
                break;
            }
        }

        if terminator.is_none() {
            return Err(Error::new(ErrorKind::Other, format!("Block -> 0x{:x}: has no end instruction", address)));
        }

        return Ok(Self {
            address: address,
            cfg: cfg,
            terminator: terminator.unwrap(),
        });
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

    pub fn process(&self) -> BlockJson {
        BlockJson {
            address: self.address,
            type_: "block".to_string(),
            next: self.terminator.next(),
            to: self.terminator.to(),
            edges: self.edges(),
            signature: self.signature(),
            prologue: self.is_prologue(),
            conditional: self.terminator.is_conditional,
            size: self.size(),
            bytes: Binary::to_hex(&self.bytes()),
            instructions: self.instruction_count(),
            functions: self.functions(),
            entropy: self.entropy(),
            sha256: self.sha256(),
            minhash: self.minhash(),
            tlsh: self.tlsh(),
            contiguous: true,
            file: self.file(),
            tags: self.cfg.options.tags.clone(),
        }
    }

    pub fn file(&self) -> Option<FileJson> {
        Some(File::new(self.cfg.options.clone()).process())
    }

    pub fn is_prologue(&self) -> bool {
        if let Some(entry) =  self.cfg.instructions.get(&self.address) {
            return entry.value().is_prologue;
        }
        return false;
    }

    pub fn edges(&self) -> usize {
        return self.terminator.edges;
    }

    pub fn next(&self) -> Option<u64> {
        if self.terminator.is_return { return None; }
        if self.terminator.is_trap { return None; }
        if !self.terminator.is_conditional { return None; }
        self.terminator.next()
    }

    pub fn to(&self) -> BTreeSet<u64> {
        self.terminator.to()
    }

    pub fn blocks(&self) -> BTreeSet<u64> {
        let mut result = BTreeSet::new();
        for item in self.to().iter().map(|ref_multi| *ref_multi).chain(self.next()) {
            result.insert(item);
        }
        result
    }

    pub fn signature(&self) -> SignatureJson {
        Signature::new(self.address, self.end(), &self.cfg, self.cfg.options.clone()).process()
    }

    pub fn functions(&self) -> BTreeMap<u64, u64> {
        let mut result = BTreeMap::<u64, u64>::new();
        for entry in self.cfg.instructions.range(self.address..=self.terminator.address){
            let instruction = entry.value();
            for function_address in instruction.functions.clone() {
                result.insert(instruction.address, function_address);
            }
        }
        return result;
    }

    pub fn entropy(&self) -> Option<f64> {
        if !self.cfg.options.enable_entropy { return None; }
        return Binary::entropy(&self.bytes());
    }

    pub fn tlsh(&self) -> Option<String> {
        if !self.cfg.options.enable_tlsh { return None; }
        return TLSH::new(&self.bytes(), self.cfg.options.tlsh_mininum_byte_size).hexdigest();
    }

    pub fn minhash(&self) -> Option<String> {
        if !self.cfg.options.enable_minhash { return None; }
        if self.bytes().len() > self.cfg.options.minhash_maximum_byte_size { return None; }
        return MinHash32::new(
            &self.bytes(),
            self.cfg.options.minhash_number_of_hashes,
            self.cfg.options.minhash_shingle_size,
            self.cfg.options.minhash_seed
        ).hexdigest();
    }

    pub fn sha256(&self) -> Option<String> {
        if !self.cfg.options.enable_sha256 { return None; }
        return SHA256::new(&self.bytes()).hexdigest();
    }

    pub fn size(&self) -> usize {
        self.bytes().len()
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for entry in self.cfg.instructions.range(self.address..=self.terminator.address){
            let instruction = entry.value();
            result.extend(instruction.bytes.clone());
        }
        return result;
    }

    pub fn instruction_count(&self) -> usize {
        let mut result: usize = 0;
        for _ in self.cfg.instructions.range(self.address..=self.terminator.address){
            result += 1;
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn end(&self) -> u64 {
        return self.terminator.address;
    }

}