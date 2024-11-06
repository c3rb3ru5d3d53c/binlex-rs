use std::collections::BTreeSet;

use crate::models::cfg::instruction::Instruction;
use crossbeam::queue::SegQueue;
use crossbeam_skiplist::SkipMap;
use crossbeam_skiplist::SkipSet;

#[derive(Clone)]
pub struct GraphOptions {
    pub enable_minhash: bool,
    pub minhash_maximum_byte_size: usize,
    pub minhash_number_of_hashes: usize,
    pub minhash_shingle_size: usize,
    pub minhash_seed: u64,
    pub enable_tlsh: bool,
    pub enable_sha256: bool,
    pub enable_entropy: bool,
    pub enable_feature: bool,
    pub tlsh_mininum_byte_size: usize,
    pub enable_normalized: bool,
    pub tags: Vec<String>,
}

impl GraphOptions {
    pub fn new() -> Self {
        return Self {
            enable_minhash: true,
            minhash_maximum_byte_size: 50,
            minhash_number_of_hashes: 64,
            minhash_shingle_size: 4,
            minhash_seed: 0,
            enable_entropy: true,
            enable_tlsh: true,
            enable_sha256: true,
            enable_feature: true,
            tlsh_mininum_byte_size: 50,
            enable_normalized: false,
            tags: vec![],
        };
    }
}

pub struct GraphQueue {
    pub queue: SegQueue<u64>,
    pub processed: SkipSet<u64>,
    pub valid: SkipSet<u64>,
    pub invalid: SkipSet<u64>,
}

impl GraphQueue {
    pub fn new() -> Self {
        return Self {
            queue: SegQueue::<u64>::new(),
            processed: SkipSet::<u64>::new(),
            valid: SkipSet::<u64>::new(),
            invalid: SkipSet::<u64>::new(),
        }
    }

    pub fn insert_invalid(&mut self, address: u64) {
        if !self.is_invalid(address) {
            if !self.is_valid(address) {
                self.invalid.insert(address);
            }
        }
    }

    pub fn is_invalid(&self, address: u64) -> bool {
        self.invalid.contains(&address)
    }

    #[allow(dead_code)]
    pub fn invalid(&self) -> &SkipSet<u64> {
        return &self.invalid;
    }

    pub fn valid(&self) -> &SkipSet<u64> {
        return &self.valid;
    }

    pub fn is_valid(&self, address: u64) -> bool {
        self.valid.contains(&address)
    }

    pub fn insert_valid(&mut self, address: u64) {
        if self.is_processed(address) {
            self.valid.insert(address);
        }
    }

    pub fn set_processed(&mut self, address: u64) {
        self.processed.insert(address);
    }

    pub fn is_processed(&self, address: u64) -> bool {
        self.processed.contains(&address)
    }

    pub fn enqueue_extend(&mut self, addresses: BTreeSet<u64>) {
        for address in addresses {
            self.enqueue(address);
        }
    }

    pub fn enqueue(&mut self, address: u64) -> bool {
        if self.is_processed(address) { return false; }
        self.queue.push(address);
        return true;
    }

    pub fn dequeue(&mut self) -> Option<u64> {
        self.queue.pop()
    }
}

pub struct Graph {
    pub instructions: SkipMap<u64, Instruction>,
    pub blocks: GraphQueue,
    pub functions: GraphQueue,
    pub options: GraphOptions,
}

impl Graph {
    #[allow(dead_code)]
    pub fn new() -> Self  {
        return Self{
            instructions: SkipMap::<u64, Instruction>::new(),
            blocks: GraphQueue::new(),
            functions: GraphQueue::new(),
            options: GraphOptions::new(),
        };
    }

    pub fn insert_instruction(&mut self, instruction: Instruction) {
        if !self.is_instruction_address(instruction.address) {
            self.instructions.insert(instruction.address, instruction);
        }
    }

    pub fn update_instruction(&mut self, instruction: Instruction) {
        if !self.is_instruction_address(instruction.address) { return }
        self.instructions.insert(instruction.address, instruction);
    }

    pub fn is_instruction_address(&self, address: u64) -> bool {
        self.instructions.contains_key(&address)
    }

    pub fn get_instruction(&self, address: u64) -> Option<Instruction> {
        self.instructions.get(&address).map(|entry|entry.value().clone())
    }

}