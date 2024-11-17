
use crate::models::binary::BinaryArchitecture;
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
use crate::models::controlflow::symbol::Symbol;
use crate::models::controlflow::file::FileJson;
use crate::models::controlflow::file::File;
use crate::models::hashing::sha256::SHA256;
use crate::models::hashing::tlsh::TLSH;
use crate::models::hashing::minhash::MinHash32;

/// Represents a JSON-serializable structure containing metadata about a function.
#[derive(Serialize, Deserialize)]
pub struct FunctionJson {
    /// The type of this entity, typically `"function"`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The architecture of the function.
    pub architecture: String,
    /// The starting address of the function.
    pub address: u64,
    /// The number of edges (connections) in the function.
    pub edges: usize,
    /// Indicates whether this function starts with a prologue.
    pub prologue: bool,
    /// The signature of the function in JSON format.
    pub signature: Option<SignatureJson>,
    /// The symbol names representing the function, if available.
    pub names: BTreeSet<String>,
    /// The size of the function in bytes, if available.
    pub size: Option<usize>,
    /// The raw bytes of the function in hexadecimal format, if available.
    pub bytes: Option<String>,
    /// A map of functions associated with the function.
    pub functions: BTreeMap<u64, u64>,
    /// The set of blocks contained within the function.
    pub blocks: BTreeSet<u64>,
    /// File metadata associated with the function, if available.
    pub file: Option<FileJson>,
    /// The number of instructions in the function.
    pub instructions: usize,
    /// The entropy of the function, if enabled.
    pub entropy: Option<f64>,
    /// The SHA-256 hash of the function, if enabled.
    pub sha256: Option<String>,
    /// The MinHash of the function, if enabled.
    pub minhash: Option<String>,
    /// The TLSH of the function, if enabled.
    pub tlsh: Option<String>,
    /// Indicates whether the function is contiguous.
    pub contiguous: bool,
    /// Tags associated with the function.
    pub tags: Vec<String>,
}

/// Represents a control flow function within a graph.
#[derive(Clone)]
pub struct Function <'function>{
    /// The starting address of the function.
    pub address: u64,
    /// The control flow graph this function belongs to.
    pub cfg: &'function Graph,
    /// The blocks that make up the function, mapped by their start addresses.
    pub blocks: BTreeMap<u64, Instruction>,
    /// The function symbol, if available.
    pub symbol: Option<Symbol>,
    /// A map of functions associated with this function.
    pub functions: BTreeMap<u64, u64>,
    /// The number of instructions in the function.
    pub instruction_count: usize,
    /// The number of edges (connections) in the function.
    pub edges: usize,
    /// Indicates whether this function starts with a prologue.
    pub is_prologue: bool,
    /// The size of the function in bytes.
    pub size: usize,
}

impl<'function> Function<'function> {
    /// Creates a new `Function` instance for the given address in the control flow graph.
    ///
    /// # Arguments
    ///
    /// * `address` - The starting address of the function.
    /// * `cfg` - A reference to the control flow graph the function belongs to.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Function)` if the function is valid; otherwise,
    /// returns an `Err` with an appropriate error message.
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
        let symbol = cfg.functions.get_symbol(address);

        let mut queue = GraphQueue::new();

        queue.enqueue(address);

        while let Some(block_address) = queue.dequeue() {
            queue.insert_processed(block_address);
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
            symbol: symbol,
        });
    }

    #[allow(dead_code)]
    pub fn architecture(&self) -> BinaryArchitecture {
        self.cfg.architecture
    }

    /// Processes the function into its JSON-serializable representation.
    ///
    /// # Returns
    ///
    /// Returns a `FunctionJson` struct containing metadata about the function.
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
            names: self.names(),
            architecture: self.architecture().to_string(),
        }
    }

    /// Retrieves function names from symbols, if available.
    ///
    /// # Returns
    ///
    /// Returns an `BTreeString<String>` containing names associated with symbols, if available.
    pub fn names(&self) -> BTreeSet<String> {
        if self.symbol.is_none() { return BTreeSet::<String>::new(); }
        return self.symbol.clone().unwrap().names;
    }

    /// Retrieves metadata about the file associated with this function, if available.
    ///
    /// # Returns
    ///
    /// Returns an `Option<FileJson>` containing file metadata if available, or `None` otherwise.
    pub fn file(&self) -> Option<FileJson> {
        Some(File::new(self.cfg.options.clone()).process())
    }

    /// Prints the JSON representation of the function to standard output.
    #[allow(dead_code)]
    pub fn print(&self) {
        if let Ok(json) = self.json() {
            println!("{}", json);
        }
    }

    /// Converts the function metadata into a JSON string representation.
    ///
    /// # Returns
    ///
    /// Returns `Ok(String)` containing the JSON representation, or an `Err` if serialization fails.
    pub fn json(&self) -> Result<String, Error> {
        let raw = self.process();
        let result = serde_json::to_string(&raw)?;
        Ok(result)
    }

    /// Generates the function's signature if the function is contiguous.
    ///
    /// # Returns
    ///
    /// Returns `Some(SignatureJson)` if the function is contiguous; otherwise, `None`.
    pub fn signature(&self) -> Option<SignatureJson> {
        if !self.is_contiguous() { return None; }
        return Some(Signature::new(self.address, self.end().unwrap(), &self.cfg, self.cfg.options.clone()).process());
    }

    /// Retrieves the total number of instructions in the function.
    ///
    /// # Returns
    ///
    /// Returns the number of instructions as a `usize`.
    pub fn instruction_count(&self) -> usize {
        return self.instruction_count;
    }

    /// Indicates whether this function starts with a prologue.
    ///
    /// # Returns
    ///
    /// Returns `true` if the function starts with a prologue; otherwise, `false`.
    pub fn is_prologue(&self) -> bool {
        return self.is_prologue;
    }

    /// Retrieves the set of block addresses in the function.
    ///
    /// # Returns
    ///
    /// Returns a `BTreeSet<u64>` containing the addresses of all blocks in the function.
    pub fn block_addresses(&self) -> BTreeSet<u64> {
        self.blocks().keys().cloned().collect()
    }

    /// Retrieves the number of edges (connections) in the function.
    ///
    /// # Returns
    ///
    /// Returns the number of edges as a `usize`.
    pub fn edges(&self) -> usize {
        return self.edges;
    }

    /// Converts the function's bytes to a hexadecimal string, if available.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the hexadecimal representation of the bytes, or `None` if unavailable.
    pub fn bytes_to_hex(&self) -> Option<String> {
        if let Some(bytes) = self.bytes() {
            return Some(Binary::to_hex(&bytes));
        }
        return None;
    }

    /// Retrieves the size of the function in bytes, if contiguous.
    ///
    /// # Returns
    ///
    /// Returns `Some(usize)` if the function is contiguous; otherwise, `None`.
    pub fn size(&self) -> Option<usize> {
        if !self.is_contiguous() { return None; }
        return Some(self.size);
    }

    /// Retrieves the address of the function's last instruction, if contiguous.
    ///
    /// # Returns
    ///
    /// Returns `Some(u64)` containing the address, or `None` if the function is not contiguous.
    pub fn end(&self) -> Option<u64> {
        if !self.is_contiguous() { return None; }
        self.blocks().iter().last().map(|(_, terminator)|terminator.address)
    }

    /// Retrieves the raw bytes of the function, if contiguous.
    ///
    /// # Returns
    ///
    /// Returns `Some(Vec<u8>)` containing the bytes, or `None` if the function is not contiguous.
    pub fn bytes(&self) -> Option<Vec<u8>> {
        if !self.is_contiguous() { return None; }
        let mut result = Vec::<u8>::new();
        for entry in self.cfg.instructions.range(self.address..=self.end().unwrap()) {
            let instruction = entry.value();
            result.extend(instruction.bytes.clone());
        }
        return Some(result);
    }

    /// Computes the SHA-256 hash of the function's bytes, if enabled and contiguous.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the hash, or `None` if SHA-256 is disabled or the function is not contiguous.
    pub fn sha256(&self) -> Option<String> {
        if !self.cfg.options.enable_sha256 { return None; }
        if !self.is_contiguous() { return None; }
        if let Some(bytes) = self.bytes() {
            return SHA256::new(&bytes).hexdigest();
        }
        return None;
    }

    /// Computes the entropy of the function's bytes, if enabled and contiguous.
    ///
    /// # Returns
    ///
    /// Returns `Some(f64)` containing the entropy, or `None` if entropy calculation is disabled or the function is not contiguous.
    pub fn entropy(&self) -> Option<f64> {
        if !self.cfg.options.enable_entropy { return None; }
        if !self.is_contiguous() { return None; }
        if let Some(bytes) = self.bytes() {
            return Binary::entropy(&bytes);
        }
        return None;
    }

    /// Computes the TLSH of the function's bytes, if enabled and contiguous.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the TLSH, or `None` if TLSH is disabled or the function is not contiguous.
    pub fn tlsh(&self) -> Option<String> {
        if !self.cfg.options.enable_tlsh { return None; }
        if !self.is_contiguous() { return None; }
        if let Some(bytes) = self.bytes() {
            return TLSH::new(&bytes, self.cfg.options.tlsh_mininum_byte_size).hexdigest();
        }
        return None;
    }

    /// Computes the MinHash of the function's bytes, if enabled and contiguous.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the MinHash, or `None` if MinHash is disabled or the function is not contiguous.
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

    /// Retrieves the blocks that make up the function.
    ///
    /// # Returns
    ///
    /// Returns a reference to a `BTreeMap<u64, Instruction>` containing the function's blocks.
    pub fn blocks(&self) -> &BTreeMap<u64, Instruction> {
        return &self.blocks;
    }

    /// Retrieves the functions associated with this function.
    ///
    /// # Returns
    ///
    /// Returns a `BTreeMap<u64, u64>` containing function addresses.
    pub fn functions(&self) -> BTreeMap<u64, u64> {
        return self.functions.clone();
    }

    /// Checks whether the function is contiguous in memory.
    ///
    /// # Returns
    ///
    /// Returns `true` if the function is contiguous; otherwise, `false`.
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
