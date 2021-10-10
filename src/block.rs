use std::time::{SystemTime, UNIX_EPOCH};

use crypto_hash::digest;
use serde::{Deserialize, Serialize};

use crate::{proofofwork::ProofOfWork, transaction::Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: i64,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, prev_block_hash: &[u8]) -> Self {
        let mut block = Block {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            transactions,
            prev_block_hash: prev_block_hash.to_owned(),
            hash: vec![],
            nonce: 0,
        };

        let pow = ProofOfWork::new(&block);
        let (nonce, hash) = pow.run();

        block.hash = hash;
        block.nonce = nonce;

        return block;
    }

    pub fn new_genesis_block(coinbase: &Transaction) -> Block {
        Self::new(vec![coinbase.to_owned()], &[])
    }

    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut data = vec![];

        for tx in &self.transactions {
            data.extend(&tx.id);
        }

        let tx_hash = digest(crypto_hash::Algorithm::SHA256, &data);

        return tx_hash;
    }
}
