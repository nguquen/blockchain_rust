use crate::{blockchain::Blockchain, proofofwork::ProofOfWork, transaction::Transaction};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "printchain")]
    PrintChain,

    #[structopt(name = "createblockchain")]
    CreateBlockchain {
        #[structopt(long = "address")]
        address: String,
    },

    #[structopt(name = "getbalance")]
    GetBalance {
        #[structopt(long = "address")]
        address: String,
    },

    #[structopt(name = "send")]
    Send {
        #[structopt(long = "from")]
        from: String,
        #[structopt(long = "to")]
        to: String,
        #[structopt(long = "amount")]
        amount: i32,
    },
}

pub struct Cli {}

impl Cli {
    pub fn run(&self) {
        let command = Command::from_args();
        match command {
            Command::CreateBlockchain { address } => self.create_blockchain(&address),
            Command::GetBalance { address } => self.get_balance(&address),
            Command::Send { from, to, amount } => self.send(&from, &to, amount),
            Command::PrintChain => self.print_chain(),
        }
    }

    fn send(&self, from: &str, to: &str, amount: i32) {
        let mut bc = Blockchain::new();

        let tx = Transaction::new_otxo_tx(from, to, amount, &bc);
        bc.mine_block(vec![tx]);

        println!("Success!");
    }

    fn get_balance(&self, address: &str) {
        let bc = Blockchain::new();
        let mut balance = 0;

        let utxos = bc.find_utxo(address);

        for txo in utxos {
            balance += txo.value;
        }

        println!("Balance of '{}': {}", address, balance);
    }

    fn create_blockchain(&self, address: &str) {
        Blockchain::create(address);
    }

    fn print_chain(&self) {
        let bc = Blockchain::new();

        for block in bc.iter() {
            println!("--------------------------------------------------");
            println!("prev hash: {}", hex::encode(&block.prev_block_hash));
            println!("hash: {}", hex::encode(&block.hash));

            let pow = ProofOfWork::new(&block);
            println!("pow: {}", pow.validate());
        }
    }
}
