use std::collections::BTreeSet;
use crate::models::controlflow::instruction::Instruction;
use crossbeam::queue::SegQueue;
use crossbeam_skiplist::SkipMap;
use crossbeam_skiplist::SkipSet;

/// Configuration options for the `Graph` structure, specifying settings for hashing, entropy, and other metadata.
#[derive(Clone)]
pub struct GraphOptions {
    /// Enables or disables MinHash computation.
    pub enable_minhash: bool,
    /// Maximum byte size for MinHash computation.
    pub minhash_maximum_byte_size: usize,
    /// Number of hashes to use for MinHash.
    pub minhash_number_of_hashes: usize,
    /// Shingle size for MinHash computation.
    pub minhash_shingle_size: usize,
    /// Seed value for MinHash.
    pub minhash_seed: u64,
    /// Enables or disables TLSH (Trend Micro Locality Sensitive Hash).
    pub enable_tlsh: bool,
    /// Enables or disables SHA-256 hash computation.
    pub enable_sha256: bool,
    /// Enables or disables entropy calculation.
    pub enable_entropy: bool,
    /// Enables or disables feature vector extraction.
    pub enable_feature: bool,
    /// Minimum byte size for TLSH computation.
    pub tlsh_mininum_byte_size: usize,
    /// Enables or disables normalization of signatures.
    pub enable_normalized: bool,
    /// SHA-256 hash of the file, if available.
    pub file_sha256: Option<String>,
    /// TLSH of the file, if available.
    pub file_tlsh: Option<String>,
    /// Size of the file in bytes, if available.
    pub file_size: Option<u64>,
    /// Tags associated with the graph.
    pub tags: Vec<String>,
}

impl GraphOptions {
    /// Creates a new `GraphOptions` instance with default values.
    ///
    /// # Returns
    ///
    /// Returns a `GraphOptions` instance with default settings.
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
            file_sha256: None,
            file_tlsh: None,
            file_size: None,
            tags: vec![],
        };
    }
}

/// Queue structure used within `Graph` for managing addresses in processing stages.
pub struct GraphQueue {
    /// Queue of addresses to be processed.
    pub queue: SegQueue<u64>,
    /// Set of addresses that have been processed.
    pub processed: SkipSet<u64>,
    /// Set of valid addresses in the graph.
    pub valid: SkipSet<u64>,
    /// Set of invalid addresses in the graph.
    pub invalid: SkipSet<u64>,
}

impl Clone for GraphQueue {
    /// Creates a clone of the `GraphQueue`, including all processed, valid, and invalid addresses.
    fn clone(&self) -> Self {
        let cloned_queue = SegQueue::new();
        let mut temp_queue = Vec::new();
        while let Some(item) = self.queue.pop() {
            cloned_queue.push(item);
            temp_queue.push(item);
        }
        for item in temp_queue {
            self.queue.push(item);
        }
        let cloned_processed = SkipSet::new();
        for item in self.processed.iter() {
            cloned_processed.insert(*item);
        }
        let cloned_valid = SkipSet::new();
        for item in self.valid.iter() {
            cloned_valid.insert(*item);
        }
        let cloned_invalid = SkipSet::new();
        for item in self.invalid.iter() {
            cloned_invalid.insert(*item);
        }
        GraphQueue {
            queue: cloned_queue,
            processed: cloned_processed,
            valid: cloned_valid,
            invalid: cloned_invalid,
        }
    }
}


impl GraphQueue {
    /// Creates a new, empty `GraphQueue` instance.
    ///
    /// # Returns
    ///
    /// Returns a new `GraphQueue` instance with empty sets and queues.
    pub fn new() -> Self {
        return Self {
            queue: SegQueue::<u64>::new(),
            processed: SkipSet::<u64>::new(),
            valid: SkipSet::<u64>::new(),
            invalid: SkipSet::<u64>::new(),
        }
    }

    /// Marks an address as invalid if it has not been marked as valid.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to mark as invalid.
    pub fn insert_invalid(&mut self, address: u64) {
        if !self.is_invalid(address) {
            if !self.is_valid(address) {
                self.invalid.insert(address);
            }
        }
    }

    /// Checks if an address is marked as invalid.
    ///
    /// # Returns
    ///
    /// Returns `true` if the address is invalid, otherwise `false`.
    pub fn is_invalid(&self, address: u64) -> bool {
        self.invalid.contains(&address)
    }

    /// Retrieves a reference to the invalid address set.
    ///
    /// # Returns
    ///
    /// Returns a reference to the `SkipSet` containing invalid addresses.
    #[allow(dead_code)]
    pub fn invalid(&self) -> &SkipSet<u64> {
        return &self.invalid;
    }

    /// Retrieves a reference to the valid address set.
    ///
    /// # Returns
    ///
    /// Returns a reference to the `SkipSet` containing valid addresses.
    pub fn valid(&self) -> &SkipSet<u64> {
        return &self.valid;
    }

    /// Retrieves a reference to the processed address set.
    ///
    /// # Returns
    ///
    /// Returns a reference to the `SkipSet` containing processed addresses.
    pub fn processed(&self) -> &SkipSet<u64> {
        return &self.processed;
    }

    /// Checks if an address is marked as valid.
    ///
    /// # Returns
    ///
    /// Returns `true` if the address is valid, otherwise `false`.
    pub fn is_valid(&self, address: u64) -> bool {
        self.valid.contains(&address)
    }

    /// Marks an address as valid if it has been processed.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to mark as valid.
    pub fn insert_valid(&mut self, address: u64) {
        if self.is_processed(address) {
            self.valid.insert(address);
        }
    }

    /// Marks multiple addresses as processed.
    ///
    /// # Arguments
    ///
    /// * `addresses` - A set of addresses to mark as processed.
    pub fn insert_processed_extend(&mut self, addresses: BTreeSet<u64>) {
        for address in addresses {
            self.insert_processed(address);
        }
    }

    /// Marks a single address as processed.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to mark as processed.
    pub fn insert_processed(&mut self, address: u64) {
        self.processed.insert(address);
    }

    /// Checks if an address has been processed.
    ///
    /// # Returns
    ///
    /// Returns `true` if the address is processed, otherwise `false`.
    pub fn is_processed(&self, address: u64) -> bool {
        self.processed.contains(&address)
    }

    /// Adds multiple addresses to the processing queue.
    ///
    /// # Arguments
    ///
    /// * `addresses` - A set of addresses to enqueue.
    pub fn enqueue_extend(&mut self, addresses: BTreeSet<u64>) {
        for address in addresses {
            self.enqueue(address);
        }
    }

    /// Adds an address to the processing queue if it hasn't been processed.
    ///
    /// # Returns
    ///
    /// Returns `true` if the address was enqueued, otherwise `false`.
    pub fn enqueue(&mut self, address: u64) -> bool {
        if self.is_processed(address) { return false; }
        self.queue.push(address);
        return true;
    }

    /// Removes an address from the processing queue.
    ///
    /// # Returns
    ///
    /// Returns `Some(u64)` containing the dequeued address if available, otherwise `None`.
    pub fn dequeue(&mut self) -> Option<u64> {
        self.queue.pop()
    }

    /// Removes all addresses from the processing queue.
    ///
    /// # Returns
    ///
    /// Returns a `BTreeSet<u64>` containing all dequeued addresses.
    pub fn dequeue_all(&mut self) -> BTreeSet<u64> {
        let mut set = BTreeSet::new();
        while let Some(address) = self.queue.pop() {
            set.insert(address);
        }
        set
    }
}

/// Represents a control flow graph with instructions, blocks, and functions.
pub struct Graph {
    /// A map of instruction addresses to `Instruction` instances.
    pub instructions: SkipMap<u64, Instruction>,
    /// Queue for managing basic blocks within the graph.
    pub blocks: GraphQueue,
    /// Queue for managing functions within the graph.
    pub functions: GraphQueue,
    /// Configuration options for the graph.
    pub options: GraphOptions,
}

impl Graph {
    /// Creates a new, empty `Graph` instance with default options.
    ///
    /// # Returns
    ///
    /// Returns a `Graph` instance with empty instructions, blocks, and functions.
    #[allow(dead_code)]
    pub fn new() -> Self  {
        return Self{
            instructions: SkipMap::<u64, Instruction>::new(),
            blocks: GraphQueue::new(),
            functions: GraphQueue::new(),
            options: GraphOptions::new(),
        };
    }

    pub fn instructions(&self) -> &SkipMap<u64, Instruction> {
        return &self.instructions;
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
    pub fn absorb(&mut self, graph: &mut Graph) {

        for entry in graph.instructions() {
            self.insert_instruction(entry.value().clone());
        }

        for entry in graph.blocks.processed() {
            self.blocks.insert_processed(entry.value().clone());
        }

        self.blocks.enqueue_extend(graph.blocks.dequeue_all());

        for entry in graph.functions.processed() {
            self.functions.insert_processed(entry.value().clone());
        }

        self.functions.enqueue_extend(graph.functions.dequeue_all());

        for entry in graph.blocks.valid() {
            self.blocks.insert_valid(entry.value().clone());
        }

        for entry in graph.blocks.invalid() {
            self.blocks.insert_invalid(entry.value().clone());
        }

        for entry in graph.functions.valid() {
            self.functions.insert_valid(entry.value().clone());
        }

        for entry in graph.functions.invalid() {
            self.functions.insert_invalid(entry.value().clone());
        }

    }

}