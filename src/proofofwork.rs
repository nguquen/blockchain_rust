use crypto_hash::digest;
use num::{BigInt, BigUint};

use crate::block::Block;

pub const TARGET_BITS: i32 = 8;

pub struct ProofOfWork<'a> {
    pub block: &'a Block,
    pub target: BigInt,
}

impl<'a> ProofOfWork<'a> {
    pub fn new(block: &'a Block) -> Self {
        let mut target = BigInt::from(1);
        target = target << (256 - TARGET_BITS);

        ProofOfWork { block, target }
    }

    pub fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let mut data = vec![];
        data.extend(&self.block.prev_block_hash);
        data.extend(&self.block.hash_transactions());
        data.extend(self.block.timestamp.to_be_bytes());
        data.extend(i64::from(TARGET_BITS).to_be_bytes());
        data.extend(nonce.to_be_bytes());

        data
    }

    pub fn run(&self) -> (i64, Vec<u8>) {
        let mut hash: Vec<u8> = vec![];
        let mut nonce = 0;

        println!("Mining a new block",);

        while nonce < i64::max_value() {
            let data = self.prepare_data(nonce);
            hash = digest(crypto_hash::Algorithm::SHA256, &data);
            print!("\r{}", hex::encode(&hash));
            let hash_int: BigInt = BigUint::from_bytes_be(&hash).into();

            if hash_int < self.target {
                break;
            } else {
                nonce = nonce + 1;
            }
        }
        println!("\n");

        (nonce, hash)
    }

    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        let hash = digest(crypto_hash::Algorithm::SHA256, &data);
        let hash_int: BigInt = BigUint::from_bytes_be(&hash).into();

        return hash_int < self.target;
    }
}
