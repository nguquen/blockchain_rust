use bincode::serialize;
use crypto_hash::digest;
use serde::{Deserialize, Serialize};

use crate::blockchain::Blockchain;

const SUBSIDY: i32 = 10;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutput {
    pub value: i32,
    pub script_pub_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
    pub txid: Vec<u8>,
    pub vout: i32,
    pub script_sig: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: Vec<u8>,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

impl TXInput {
    pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool {
        return self.script_sig == unlocking_data;
    }
}

impl TXOutput {
    pub fn can_be_unlocked_with(&self, unlocking_data: &str) -> bool {
        return self.script_pub_key == unlocking_data;
    }
}

impl Transaction {
    pub fn new(vin: Vec<TXInput>, vout: Vec<TXOutput>) -> Self {
        let mut tx = Transaction {
            id: vec![],
            vin,
            vout,
        };

        let encoded = serialize(&tx).unwrap();
        tx.id = digest(crypto_hash::Algorithm::SHA256, &encoded);

        return tx;
    }

    pub fn new_coinbase_tx(to: &str, data: &str) -> Transaction {
        let mut script_sig = data.to_owned();
        if data.is_empty() {
            script_sig = format!("Reward to '{}'", to);
        }

        let txin = TXInput {
            txid: vec![],
            vout: -1,
            script_sig,
        };

        let txout = TXOutput {
            value: SUBSIDY,
            script_pub_key: to.to_string(),
        };

        return Transaction::new(vec![txin], vec![txout]);
    }

    pub fn is_coinbase(&self) -> bool {
        return self.vin.len() == 1 && self.vin[0].txid.len() == 0 && self.vin[0].vout == -1;
    }

    pub fn new_otxo_tx(from: &str, to: &str, amount: i32, bc: &Blockchain) -> Transaction {
        let mut inputs: Vec<TXInput> = vec![];
        let mut outputs: Vec<TXOutput> = vec![];

        let (acc, valid_outputs) = bc.find_spendable_outputs(from, amount);
        if acc < amount {
            panic!("ERROR: Not enough funds");
        }

        for (tx_id, outs) in valid_outputs {
            let txid = hex::decode(tx_id).unwrap();

            for out in outs {
                inputs.push(TXInput {
                    txid: txid.to_owned(),
                    vout: out,
                    script_sig: from.to_string(),
                })
            }
        }

        outputs.push(TXOutput {
            value: amount,
            script_pub_key: to.to_string(),
        });
        if acc > amount {
            outputs.push(TXOutput {
                value: acc - amount,
                script_pub_key: from.to_string(),
            });
        }

        Transaction::new(inputs, outputs)
    }
}
