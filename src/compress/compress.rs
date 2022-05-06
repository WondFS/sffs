use crate::compress::huffman;
use crate::compress::snappy;

pub enum CompressType {
    None,
    Huffman,
    Snappy,
}

pub trait Compress {
    fn decode(&self, bytes: &[u8]) -> Vec<u8>;
    fn encode(&self, bytes: &[u8]) -> Vec<u8>;
}

pub struct CompressManager {
    
}

impl CompressManager {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn encode(&self, bytes: &[u8]) -> (Vec<u8>, CompressType) {
        todo!()
    }

    pub fn decode(&self, bytes: &[u8]) -> (Vec<u8>, CompressType) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {

    }
}