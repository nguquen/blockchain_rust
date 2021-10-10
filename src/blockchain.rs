use crate::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn add_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new(data, &prev_block.hash);
        self.blocks.push(new_block);
    }

    pub fn new() -> Self {
        Blockchain {
            blocks: vec![Block::new_genesis_block()],
        }
    }
}
