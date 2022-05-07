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
    pub snappy: snappy::Snappy,
    pub huffman: huffman::HuffmanCodec,
}

impl CompressManager {
    pub fn new() -> Self {
        Self {
            snappy: snappy::Snappy::new(),
            huffman: huffman::HuffmanCodec::new(),
        }
    }

    pub fn encode(&self, bytes: &[u8]) -> (Vec<u8>, CompressType) {
        let res = self.snappy.encode(bytes);
        (res, CompressType::Snappy)
    }

    pub fn decode(&self, bytes: &[u8], compress_type: CompressType) -> Vec<u8> {
        self.snappy.decode(bytes)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {

    }
}