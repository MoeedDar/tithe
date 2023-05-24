use ring::digest;

#[derive(thiserror::Error, Debug)]
enum TxError {
    #[error("invalid transaction")]
    InvalidTransaction    
}

#[derive(Clone)]
pub struct Tx {
    timestamp: u64,
    data: Vec<u8>,
}

impl Tx {
    pub fn new(timestamp: u64, data: Vec<u8>) -> Tx {
        Tx { timestamp, data }
    }

    pub fn get_hash(&self) -> [u8; 32] {
        digest::digest(&digest::SHA256, &self.get_bytes())
            .as_ref()[..32]
            .try_into()
            .unwrap()
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        [&self.timestamp.to_be_bytes(), self.data.as_slice()].concat()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Tx {
        let timestamp = u64::from_be_bytes(bytes[..8].try_into().unwrap());
        let data = bytes[8..].to_vec();
        Tx{ timestamp, data}
    }
}
