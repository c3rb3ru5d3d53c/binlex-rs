use std::collections::BTreeSet;
use crate::binary::BinaryArchitecture;
use crate::controlflow::Instruction;
use crossbeam::queue::SegQueue;
use crossbeam_skiplist::SkipMap;
use crossbeam_skiplist::SkipSet;
use crate::controlflow::Symbol;
use crate::config::Config;

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
    /// Map of symbol addresses in the graph.
    pub symbols: SkipMap<u64, Symbol>,
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
        let cloned_symbols = SkipMap::<u64, Symbol>::new();
        for entry in self.symbols.iter() {
            cloned_symbols.insert(*entry.key(), entry.value().clone());
        }
        GraphQueue {
            queue: cloned_queue,
            processed: cloned_processed,
            valid: cloned_valid,
            invalid: cloned_invalid,
            symbols: cloned_symbols,
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
            symbols: SkipMap::<u64, Symbol>::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_symbol(&self, address: u64) -> Option<Symbol> {
        let entry = self.symbols.get(&address);
        if entry.is_none() {
            return None;
        }
        Some(entry.unwrap().value().clone())
    }

    #[allow(dead_code)]
    pub fn insert_symbol(&self, mut symbol: Symbol) {
        if let Some(existing) = self.symbols.get(&symbol.address) {
            symbol.insert_name_entend(existing.value().names.clone());
            return;
        }
        self.symbols.insert(symbol.address, symbol);
    }

    #[allow(dead_code)]
    pub fn insert_symbols_extend(&self, symbols: Vec<Symbol>) {
        for symbol in symbols {
            self.insert_symbol(symbol);
        }
    }

    #[allow(dead_code)]
    pub fn is_symbol(&self, address: u64) -> bool {
        self.symbols.contains_key(&address)
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
pub struct Graph <'graph> {
    /// The Instruction Architecture
    pub architecture: BinaryArchitecture,
    /// A map of instruction addresses to `Instruction` instances.
    pub instructions: SkipMap<u64, Instruction>,
    /// Queue for managing basic blocks within the graph.
    pub blocks: GraphQueue,
    /// Queue for managing functions within the graph.
    pub functions: GraphQueue,
    /// Configuration options for the graph.
    //pub options: GraphOptions,
    /// Configuration
    pub config: &'graph Config,
}

impl <'graph> Graph <'graph> {
    /// Creates a new, empty `Graph` instance with default options.
    ///
    /// # Returns
    ///
    /// Returns a `Graph` instance with empty instructions, blocks, and functions.
    #[allow(dead_code)]
    pub fn new(architecture: BinaryArchitecture, config: &'graph Config) -> Self  {
        return Self{
            architecture: architecture,
            instructions: SkipMap::<u64, Instruction>::new(),
            blocks: GraphQueue::new(),
            functions: GraphQueue::new(),
            config: config,
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