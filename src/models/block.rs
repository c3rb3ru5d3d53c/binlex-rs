use crate::models::instruction::Instruction;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{BTreeMap, HashSet};
use std::io::Error;
use std::io::ErrorKind;
use crate::models::binary::Binary;
use crate::models::cfg::CFG;
use crate::models::signature::Signature;
use crate::models::signature::SignatureJson;

#[derive(Serialize, Deserialize)]
pub struct BlockJson {
    #[serde(rename = "type")]
    pub type_: String,
    pub address: u64,
    pub next: Option<u64>,
    pub to: HashSet<u64>,
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
    pub tags: Vec<String>,
}

pub struct Block <'block>{
    pub address: u64,
    pub cfg: &'block CFG,
    pub terminator: &'block Instruction,
}

impl<'block> Block<'block> {

    pub fn new(address: u64, cfg: &'block CFG) -> Result<Self, Error> {

        if !cfg.blocks.is_valid(address) {
            return Err(Error::new(ErrorKind::Other, format!("Block -> 0x{:x}: is not valid", address)));
        }

        let mut terminator: Option<&Instruction> = None;

        for (_, instruction) in cfg.instructions.range(address..){
            let previous_address: Option<u64> = None;
            if let Some(previous_address) = previous_address{
                if instruction.address != previous_address {
                    return Err(Error::new(ErrorKind::Other, format!("Block -> 0x{:x}: is not contiguous", address)));
                }
            }
            if instruction.is_jump
                || instruction.is_trap
                || instruction.is_return
                || (address != instruction.address && instruction.is_block_start) {
                terminator = Some(instruction);
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
            to: self.terminator.to.clone(),
            edges: self.edges(),
            signature: self.signature(),
            prologue: self.is_prologue(),
            conditional: self.terminator.is_conditional,
            size: self.size(),
            bytes: Binary::to_hex(&self.bytes()),
            instructions: self.instructions().len(),
            functions: self.functions(),
            entropy: self.entropy(),
            sha256: self.sha256(),
            minhash: self.minhash(),
            tlsh: self.tlsh(),
            contiguous: true,
            tags: self.cfg.options.tags.clone(),
        }
    }

    pub fn is_prologue(&self) -> bool {
        self.instructions().first().map(|instruction|instruction.is_prologue).unwrap()
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

    pub fn to(&self) -> HashSet<u64> {
        self.terminator.to.clone()
    }

    pub fn blocks(&self) -> HashSet<u64> {
        self.to().iter().cloned().chain(self.next()).collect()
    }

    pub fn signature(&self) -> SignatureJson {
        Signature::new(&self.instructions(), self.cfg.options.clone()).process()
    }

    pub fn functions(&self) -> BTreeMap<u64, u64> {
        self.instructions()
            .iter()
            .flat_map(|instruction| instruction.functions.clone().into_iter()
            .map(move |function_address| (instruction.address, function_address)))
            .collect()
    }

    pub fn entropy(&self) -> Option<f64> {
        if !self.cfg.options.enable_entropy { return None; }
        return Binary::entropy(&self.bytes());
    }

    pub fn tlsh(&self) -> Option<String> {
        if !self.cfg.options.enable_tlsh { return None; }
        return Binary::tlsh(&self.bytes(), self.cfg.options.tlsh_mininum_byte_size);
    }

    pub fn minhash(&self) -> Option<String> {
        if !self.cfg.options.enable_minhash { return None; }
        return Binary::minhash(
            self.cfg.options.minhash_maximum_byte_size,
            self.cfg.options.minhash_number_of_hashes,
            self.cfg.options.minhash_shingle_size,
            self.cfg.options.minhash_seed,
            &self.bytes());
    }

    pub fn sha256(&self) -> Option<String> {
        if !self.cfg.options.enable_sha256 { return None; }
        return Binary::sha256(&self.bytes());
    }

    pub fn size(&self) -> usize {
        self.bytes().len()
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.instructions()
            .iter()
            .flat_map(|instruction| instruction.bytes.clone())
            .collect()
    }

    pub fn instructions(&self) -> Vec<&Instruction> {
        self.cfg
            .instructions
            .range(self.address..=self.terminator.address)
            .map(|(_, instr)| instr)
            .collect()
    }

    #[allow(dead_code)]
    pub fn end(&self) -> u64 {
        return self.terminator.address;
    }

}