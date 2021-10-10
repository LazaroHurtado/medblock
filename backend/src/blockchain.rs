use super::block::Block;
use super::block::BlockData;
use std::fmt;
use std::sync::{Arc, Mutex};

pub struct Blockchain {
    pub chain: Mutex<Chain>,
}

impl Blockchain {
    pub fn new() -> Self {
        let chain = Chain::new();
        Self {
            chain: Mutex::new(chain),
        }
    }

    pub fn append_block(&self, block: Block) {
        let mut lock = self.chain.lock().unwrap();
        lock.append_block(block);
    }

    pub fn len(&self) -> usize {
        let lock = self.chain.lock().unwrap();
        lock.get_amount_of_blocks()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_most_recent_block(&self) -> Option<Block> {
        let lock = self.chain.lock().unwrap();
        lock.get_most_recent_block()
    }

    pub fn check_validity(&self, company: String, vaccine: String) -> bool {
        let lock = self.chain.lock().unwrap();
        lock.check_validity(company, vaccine)
    }
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let last_block = self.get_most_recent_block();
        match last_block {
            Some(last_block) => write!(f, "{}", last_block.get_hash_address()),
            None => write!(f, "No last block available"),
        }
    }
}

pub struct Chain {
    pub block_list: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        let genesis_block_data = BlockData::default();
        let genesis_block = Block::new(genesis_block_data, "0".to_string());

        Self {
            block_list: vec![genesis_block],
        }
    }

    pub fn append_block(&mut self, block: Block) {
        self.block_list.push(block);
    }

    pub fn get_most_recent_block(&self) -> Option<Block> {
        Some(self.block_list.last().unwrap().clone())
    }

    pub fn get_amount_of_blocks(&self) -> usize {
        self.block_list.len()
    }

    pub fn check_validity(&self, company: String, vaccine: String) -> bool {
        let mut valid: bool = false;
        for block in &self.block_list {
            if block.data.from_address == company && block.data.supply_name == vaccine {
                valid = true;
                break;
            }
        }
        valid
    }
}
