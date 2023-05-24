use std::path::PathBuf;

use crate::{AppendLog, MerkleTree, Tx, append_log::AppendLogError};

#[derive(thiserror::Error, Debug)]
pub enum DBError {
    #[error("transaction pending")]
    TransactionPending,
    
    #[error("no transaction pending")]
    NoTransactionPending,

    #[error(transparent)]
    AppendLogError(#[from] AppendLogError),
}

pub struct DB {
    append_log: AppendLog,
    merkle_tree: MerkleTree,
    tx: Option<Tx>,
}

impl DB {
    pub fn new(dir: PathBuf) -> DB {
        DB {
            append_log: AppendLog::new(dir).unwrap(),
            merkle_tree: MerkleTree::new(),
            tx: None,
        }
    }

    pub fn append(&mut self, tx: Tx) -> Result<Tx, DBError> {
        self.tx
            .take()
            .map_or_else(|| {
                self.tx = Some(tx.clone());
                Ok(tx)
            }, |_| Err(DBError::TransactionPending))
    }

    pub fn retrieve(&mut self, index: u64) -> Result<Tx, DBError> {
        self.append_log
            .at(index)
            .map(|raw| {
                Tx::from_bytes(raw.clone())
            })
            .map_err(Into::into)
    }

    pub fn commit(&mut self) -> Result<Tx, DBError> {
        self.tx
            .take()
            .map_or_else(|| Err(DBError::NoTransactionPending), |tx| {
                let mut merkle_tree = self.merkle_tree.clone(); // Very bad fix pls
                merkle_tree.append(tx.get_hash());
                self.append_log.append(tx.get_bytes().as_slice())?;
                self.merkle_tree = merkle_tree;
                Ok(tx)
            })
    }

    pub fn rollback(&mut self) -> Result<Tx, DBError> {
        self.tx
            .take()
            .map_or_else(|| Err(DBError::NoTransactionPending), |tx| {
                self.tx = None;
                Ok(tx)
            })
    }
}
