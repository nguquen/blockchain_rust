use std::time::{SystemTime, UNIX_EPOCH};

use crypto_hash::digest;
use num::{BigInt, BigUint};

const TARGET_BITS: i32 = 24;

pub struct Block {
    pub timestamp: u64,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: i64,
}

pub struct ProofOfWork<'a> {
    pub block: &'a Block,
    pub target: BigInt,
}

impl Block {
    pub fn new(data: &str, prev_block_hash: &[u8]) -> Self {
        let mut block = Block {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data: data.as_bytes().to_owned(),
            prev_block_hash: prev_block_hash.to_owned(),
            hash: vec![],
            nonce: 0,
        };

        let pow = block.new_proof_of_work();
        let (nonce, hash) = pow.run();

        block.hash = hash;
        block.nonce = nonce;

        return block;
    }

    pub fn new_genesis_block() -> Block {
        Self::new("Genesis Block", &[])
    }

    pub fn new_proof_of_work(&self) -> ProofOfWork {
        let mut target = BigInt::from(1);
        target = target << (256 - TARGET_BITS);

        ProofOfWork {
            block: &self,
            target,
        }
    }
}

impl<'a> ProofOfWork<'a> {
    pub fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let mut data = vec![];
        data.extend(&self.block.prev_block_hash);
        data.extend(&self.block.data);
        data.extend(self.block.timestamp.to_be_bytes());
        data.extend(i64::from(TARGET_BITS).to_be_bytes());
        data.extend(nonce.to_be_bytes());

        data
    }

    pub fn run(&self) -> (i64, Vec<u8>) {
        let mut hash_int: BigInt;
        let mut hash: Vec<u8> = vec![];
        let mut nonce = 0;

        println!(
            "Mining the block containing {:?}",
            String::from_utf8(self.block.data.to_owned()).unwrap()
        );

        while nonce < i64::max_value() {
            let data = self.prepare_data(nonce);
            hash = digest(crypto_hash::Algorithm::SHA256, &data);
            print!("\r{}", hex::encode(&hash));
            hash_int = BigUint::from_bytes_be(&hash).into();

            if hash_int < self.target {
                break;
            } else {
                nonce = nonce + 1;
            }
        }
        println!("\n");

        (nonce, hash)
    }
}
