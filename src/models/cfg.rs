use std::collections::{HashMap, BTreeMap};
use crate::models::block::Block;
use crate::models::function::Function;

pub struct CFG {
    pub functions: BTreeMap<u64, Function>,
}

impl CFG {
    #[allow(dead_code)]
    pub fn new() -> Self  {
        return Self{
            functions: BTreeMap::<u64, Function>::new(),
        };
    }

    #[allow(dead_code)]
    pub fn functions(&self) -> Vec<&Function> {
        self.functions.values().collect()
    }

    #[allow(dead_code)]
    pub fn blocks(&self) -> Vec<&Block> {
        let mut map = HashMap::<u64, &Block>::new();
        for function in self.functions() {
            for block in function.blocks() {
                map.entry(block.address)
                    .and_modify(|existing_block| {
                        if block.size() < existing_block.size() {
                            *existing_block = block;
                        }
                    })
                    .or_insert(block);
            }
        }
        map.into_values().collect()
    }

    #[allow(dead_code)]
    pub fn print_functions(&self) {
        for function in self.functions() {
            function.print();
        }
    }

    #[allow(dead_code)]
    pub fn print_blocks(&self) {
        for block in self.blocks() {
            block.print();
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        self.print_functions();
        self.print_blocks();
    }

}