use std::time::{SystemTime, UNIX_EPOCH};

use crypto_hash::digest;

pub struct Block {
    pub timestamp: u64,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
}

impl Block {
    pub fn set_hash(&mut self) {
        let mut headers = vec![];
        headers.extend(&self.prev_block_hash);
        headers.extend(&self.data);
        headers.extend(self.timestamp.to_string().as_bytes());

        let hash = digest(crypto_hash::Algorithm::SHA256, &headers);
        self.hash = hash;
    }

    pub fn new(data: &str, prev_block_hash: Vec<u8>) -> Self {
        let mut block = Block {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data: data.as_bytes().to_vec(),
            prev_block_hash,
            hash: vec![],
        };

        block.set_hash();

        return block;
    }

    pub fn new_genesis_block() -> Block {
        Self::new("Genesis Block", vec![])
    }
}
