use std::io::Error;
use std::collections::{HashSet, BTreeMap};
use serde::{Deserialize, Serialize};
use serde_json;
use crate::models::binary::Binary;
use crate::models::instruction::Instruction;
use crate::models::signature::{Signature, SignatureJson};
use crate::models::disassembler::DisassemblerOptions;

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

pub struct Block {
    pub address: u64,
    pub functions: BTreeMap<u64, u64>,
    pub error: Option<Error>,
    pub next: Option<u64>,
    pub to: HashSet<u64>,
    pub edges: usize,
    pub instructions: BTreeMap<u64, Instruction>,
    pub prologue: bool,
    pub conditional: bool,
    pub options: DisassemblerOptions,
}

impl Block {
    #[allow(dead_code)]
    pub fn new(address: u64, options: DisassemblerOptions) -> Result<Self, Error> {
        Ok(Self{
            address: address,
            error: None,
            next: None,
            to: HashSet::<u64>::new(),
            functions: BTreeMap::<u64, u64>::new(),
            edges: 0,
            instructions: BTreeMap::<u64, Instruction>::new(),
            prologue: false,
            conditional: false,
            options: options,
        })
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if let Ok(json) = self.json() {
            println!("{}", json);
        }
    }

    #[allow(dead_code)]
    pub fn process(&self) -> BlockJson {
        BlockJson {
            address: self.address,
            type_: "block".to_string(),
            next: self.next,
            to: self.to.clone(),
            edges: self.edges,
            prologue: self.prologue,
            conditional: self.conditional,
            signature: self.signature(),
            size: self.size(),
            bytes: self.to_hex(),
            functions: self.functions().clone(),
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
    pub fn has_return(&self) -> bool {
        self.instructions().iter().any(|instruction| instruction.is_ret)
    }

    #[allow(dead_code)]
    pub fn json(&self) -> Result<String, Error> {
        let raw = self.process();
        let result = serde_json::to_string(&raw)?;
        Ok(result)
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
    pub fn sha256(&self) -> Option<String> {
        if !self.options.enable_sha256 { return None; }
        Binary::sha256(&self.bytes())
    }

    #[allow(dead_code)]
    pub fn entropy(&self) -> Option<f64> {
        if !self.options.enable_entropy { return None; }
        Binary::entropy(&self.bytes())
    }

    #[allow(dead_code)]
    pub fn is_contiguous(&self) -> bool {
        let instructions = self.instructions();
        for i in 0..instructions.len() - 1 {
            let current_instr = &instructions[i];
            let next_instr = &instructions[i + 1];
            if current_instr.address + current_instr.size() as u64 != next_instr.address {
                return false;
            }
        }
        true
    }

    #[allow(dead_code)]
    pub fn functions(&self) -> &BTreeMap<u64, u64> {
        &self.functions
    }

    #[allow(dead_code)]
    pub fn to_hex(&self) -> String {
        self.bytes().iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    }

    #[allow(dead_code)]
    pub fn is_conditional(&self) -> bool {
        self.conditional
    }

    #[allow(dead_code)]
    pub fn is_unconditional(&self) -> bool {
        !self.conditional
    }

    #[allow(dead_code)]
    pub fn instructions(&self) -> Vec<&Instruction> {
        self.instructions
            .values()
            .collect()
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.instructions()
            .iter()
            .map(|instruction| instruction.size())
            .sum()
    }

    pub fn signature(&self) -> SignatureJson {
        Signature::new(&self.instructions(), self.options.clone()).process()
    }

    #[allow(dead_code)]
    pub fn bytes(&self) -> Vec<u8> {
        self.instructions()
            .iter()
            .flat_map(|instruction| instruction.bytes.clone())
            .collect()
    }

    #[allow(dead_code)]
    pub fn tlsh(&self) -> Option<String> {
        if !self.options.enable_tlsh { return None; }
        Binary::tlsh(&self.bytes(), self.options.tlsh_mininum_byte_size)
    }

    #[allow(dead_code)]
   pub fn hexdump(&self) -> String{
        Binary::hexdump(&self.bytes(), self.address)
   }

}