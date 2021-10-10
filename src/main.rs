use blockchain_rust::blockchain::Blockchain;

fn main() {
    let mut bc = Blockchain::new();
    bc.add_block("Send 1 BTC to Ivan");
    bc.add_block("Send 2 more BTC to Ivan");

    for block in bc.blocks {
        println!("----------------------------------------");
        println!("prev hash: {}", hex::encode(block.prev_block_hash));
        println!("data: {}", String::from_utf8(block.data).unwrap());
        println!("hash: {}", hex::encode(block.hash));
    }
}
