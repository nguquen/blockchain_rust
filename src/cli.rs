use crate::blockchain::Blockchain;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "addblock")]
    AddBlock {
        #[structopt(long = "data")]
        data: String,
    },

    #[structopt(name = "printchain")]
    PrintChain,
}

pub struct Cli<'a> {
    pub bc: &'a Blockchain,
}

impl<'a> Cli<'a> {
    pub fn run(&self) {
        let command = Command::from_args();
        match command {
            Command::AddBlock { data } => {
                self.bc.add_block(data.as_str());
            }
            Command::PrintChain => {
                for block in self.bc.iter() {
                    println!("--------------------------------------------------");
                    println!("prev hash: {}", hex::encode(&block.prev_block_hash));
                    println!(
                        "data: {}",
                        String::from_utf8(block.data.to_owned()).unwrap()
                    );
                    println!("hash: {}", hex::encode(&block.hash));

                    let pow = block.new_proof_of_work();
                    println!("pow: {}", pow.validate());
                }
            }
        }
    }
}
