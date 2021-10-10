use bincode::{deserialize, serialize};
use sled::Db;

use crate::block::Block;

pub struct Blockchain {
    pub tip: Vec<u8>,
    pub db: Db,
}

pub struct BlockchainIterator<'a> {
    pub current_hash: Vec<u8>,
    pub db: &'a Db,
}

impl Blockchain {
    pub fn add_block(&self, data: &str) {
        let last_hash = self.db.get("l").unwrap().unwrap().to_vec();
        let new_block = Block::new(data, &last_hash);

        self.db
            .insert(&new_block.hash, serialize(&new_block).unwrap())
            .unwrap();
        self.db.insert("l", new_block.hash.to_owned()).unwrap();
        self.db.flush().unwrap();
    }

    pub fn new() -> Self {
        let db = sled::open("data/blockchain.db").unwrap();
        let tip: Vec<u8>;

        if db.is_empty() {
            let genesis = Block::new_genesis_block();
            db.insert(&genesis.hash, serialize(&genesis).unwrap())
                .unwrap();
            db.insert("l", genesis.hash.to_owned()).unwrap();
            db.flush().unwrap();
            tip = genesis.hash;
        } else {
            tip = db.get("l").unwrap().unwrap().to_vec();
        }

        Blockchain { tip, db }
    }

    pub fn iter(&self) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: self.tip.to_owned(),
            db: &self.db,
        }
    }
}

impl<'a> Iterator for BlockchainIterator<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encoded_block) = self.db.get(&self.current_hash) {
            return match encoded_block {
                Some(b) => {
                    if let Ok(block) = deserialize::<Block>(&b) {
                        self.current_hash = block.prev_block_hash.to_owned();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None,
            };
        }
        None
    }
}
