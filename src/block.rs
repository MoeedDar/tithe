use crate::{HashType, MerkleTree, Tx};

pub const BLOCK_SIZE: usize = 2048;

#[derive(thiserror::Error, Debug)]
pub enum BlockError {
    #[error("block size exceeded")]
    BlockSizeExceeded
}

pub struct Block {
    index: u64,
    prev_hash: HashType,
    merkle_tree: MerkleTree,
    txs: Vec<Tx>,
}

impl Block {
    pub fn new(index: u64, prev_hash: HashType) -> Block {
        Block {
            index,
            prev_hash,
            merkle_tree: MerkleTree::new(),
            txs: Vec::with_capacity(BLOCK_SIZE),
        }
    }

    pub fn get_hash() {}

    pub fn get_root(&self) -> HashType {
        self.merkle_tree.root()
    }

    pub fn append(&mut self, tx: Tx) -> Result<(), BlockError>{
        if self.is_full() {
            return Err(BlockError::BlockSizeExceeded)
        }

        self.merkle_tree.append(tx.get_hash());
        self.append(tx);
        
        Ok(())
    }

    pub fn is_full(&self) -> bool {
        self.txs.len() >= BLOCK_SIZE
    }
}
