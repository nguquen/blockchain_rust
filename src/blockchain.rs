use std::{collections::HashMap, process::exit};

use bincode::{deserialize, serialize};
use sled::Db;

use crate::{
    block::Block,
    transaction::{TXOutput, Transaction},
};

const GENESIS_COINBASE_DATA: &str =
    "The Times 03/Jan/2009 Chancellor on brink of second bailout for bank";

const DB_PATH: &str = "data/blockchain.db";

pub struct Blockchain {
    pub tip: Vec<u8>,
    pub db: Db,
}

pub struct BlockchainIterator<'a> {
    pub current_hash: Vec<u8>,
    pub db: &'a Db,
}

impl Blockchain {
    pub fn mine_block(&mut self, transactions: Vec<Transaction>) {
        let last_hash = self.db.get("l").unwrap().unwrap().to_vec();
        let new_block = Block::new(transactions, &last_hash);

        self.db
            .insert(&new_block.hash, serialize(&new_block).unwrap())
            .unwrap();
        self.db.insert("l", new_block.hash.to_owned()).unwrap();
        self.db.flush().unwrap();

        self.tip = new_block.hash;
    }

    pub fn new() -> Self {
        let db = sled::open(DB_PATH).unwrap();

        if db.is_empty() {
            println!("No existing blockchain found. Create one first.");
            exit(1);
        }

        let tip = db.get("l").unwrap().unwrap().to_vec();

        Blockchain { tip, db }
    }

    pub fn create(address: &str) -> Self {
        let db = sled::open(DB_PATH).unwrap();

        if !db.is_empty() {
            println!("Blockchain already exists.");
            exit(1);
        }

        let coinbase = Transaction::new_coinbase_tx(address, GENESIS_COINBASE_DATA);
        let genesis = Block::new_genesis_block(&coinbase);
        db.insert(&genesis.hash, serialize(&genesis).unwrap())
            .unwrap();
        db.insert("l", genesis.hash.to_owned()).unwrap();
        db.flush().unwrap();

        Blockchain {
            tip: genesis.hash,
            db,
        }
    }

    pub fn iter(&self) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: self.tip.to_owned(),
            db: &self.db,
        }
    }

    pub fn find_unspent_transactions(&self, address: &str) -> Vec<Transaction> {
        let mut unspent_txs: Vec<Transaction> = vec![];
        let mut spent_txos: HashMap<String, Vec<i32>> = HashMap::new();

        for block in self.iter() {
            for tx in block.transactions {
                let tx_id = hex::encode(&tx.id);

                for (out_idx, txo) in tx.vout.iter().enumerate() {
                    if let Some(out_idxs) = spent_txos.get(&tx_id) {
                        if out_idxs.contains(&(out_idx as i32)) {
                            continue;
                        }
                    }

                    if txo.can_be_unlocked_with(address) {
                        unspent_txs.push(tx.to_owned());
                    }
                }

                if tx.is_coinbase() {
                    continue;
                }

                for txi in &tx.vin {
                    if txi.can_unlock_output_with(address) {
                        let txin_id = hex::encode(&txi.txid);
                        match spent_txos.get_mut(&txin_id) {
                            Some(v) => {
                                v.push(txi.vout);
                            }
                            None => {
                                spent_txos.insert(txin_id, vec![txi.vout]);
                            }
                        }
                    }
                }
            }
        }

        unspent_txs
    }

    pub fn find_utxo(&self, address: &str) -> Vec<TXOutput> {
        let mut utxos: Vec<TXOutput> = vec![];
        let unspent_txs = self.find_unspent_transactions(address);

        for tx in unspent_txs {
            for txo in tx.vout {
                if txo.can_be_unlocked_with(address) {
                    utxos.push(txo);
                }
            }
        }

        utxos
    }

    pub fn find_spendable_outputs(
        &self,
        address: &str,
        amount: i32,
    ) -> (i32, HashMap<String, Vec<i32>>) {
        let unspent_txs = self.find_unspent_transactions(address);
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut acc = 0;

        for tx in unspent_txs {
            let tx_id = hex::encode(&tx.id);

            for (txo_idx, txo) in tx.vout.iter().enumerate() {
                if txo.can_be_unlocked_with(address) && acc < amount {
                    acc += txo.value;

                    match unspent_outputs.get_mut(&tx_id) {
                        Some(v) => {
                            v.push(txo_idx as i32);
                        }
                        None => {
                            unspent_outputs.insert(tx_id.to_string(), vec![txo_idx as i32]);
                        }
                    }

                    if acc >= amount {
                        break;
                    }
                }
            }
        }

        (acc, unspent_outputs)
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
