use crate::utils;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from("genesis"),
            data: String::from("genesis"),
            nonce: 0,
            hash: String::from("0"),
        }
    }
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now();
        let (nonce, hash) = utils::mine_block(id, now.timestamp(), &previous_hash, &data);
        Self {
            id,
            hash,
            timestamp: now.timestamp(),
            previous_hash,
            data,
            nonce,
        }
    }
    pub fn is_valid(&self, previous_block: &Block) -> bool {
        if self.previous_hash != previous_block.hash {
            log::warn!("block with id: {} has wrong previous hash", self.id);
        } else if !utils::hash2binary(&hex::decode(&self.hash).expect("can decode from hex"))
            .starts_with(utils::DIFFICULTY_PREFIX)
        {
            log::warn!("block with id: {} has invalid dificulty", self.id,);
        } else if self.id != previous_block.id + 1 {
            log::warn!(
                "block with id: {} is not the next block after the latest: {}",
                self.id,
                previous_block.id
            );
            return false;
        } else if hex::encode(utils::calculate_hash(
            self.id,
            self.timestamp,
            &self.previous_hash,
            &self.data,
            self.nonce,
        )) != self.hash
        {
            log::warn!("block with id: {} has invalid hash", self.id);
            return false;
        }
        true
    }
}

#[derive(PartialEq, Debug)]
pub struct Chain {
    pub blocks: Vec<Block>,
}

impl PartialOrd for Chain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.is_valid(), other.is_valid()) {
            (true, true) => self.blocks.len().partial_cmp(&other.blocks.len()),
            (true, false) => Some(Ordering::Greater),
            (false, true) => Some(Ordering::Less),
            (false, false) => None,
        }
    }
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            blocks: vec![Block::default()],
        }
    }
}

impl Chain {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_valid(&self) -> bool {
        for [previous, current] in self.blocks.windows(2).flat_map(<&[Block; 2]>::try_from) {
            if !current.is_valid(previous) {
                return false;
            }
        }
        true
    }
    pub fn add_block(&mut self, block: Block) {
        let previous = self.blocks.last().unwrap();
        if !block.is_valid(previous) {
            panic!("could not add block - invalid");
        }
        self.blocks.push(block);
    }
    pub fn choose_chain(&mut self, remote: Chain) {
        match (*self).partial_cmp(&remote) {
            None => panic!("local and remote chains are both invalid"),
            Some(Ordering::Less) => self.blocks = remote.blocks,
            _ => (),
        }
    }
}
