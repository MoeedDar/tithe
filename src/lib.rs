mod append_log;
mod block;
mod db;
mod merkle_tree;
mod tx;

pub use crate::append_log::AppendLog;
pub use crate::block::Block;
pub use crate::db::DB;
pub use crate::merkle_tree::MerkleTree;
pub use crate::tx::Tx;

pub type HashType = [u8; 32];