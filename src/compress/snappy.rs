use crate::compress::compress;

pub struct Snappy {

}

impl Snappy {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl compress::Compress for Snappy {
    fn encode(&self, bytes: &[u8]) -> Vec<u8> {
        use snap::write;
        use std::io::Write;

        let mut wtr = write::FrameEncoder::new(vec![]);
        wtr.write_all(bytes).unwrap();
        wtr.into_inner().unwrap()
    }
    
    fn decode(&self, bytes: &[u8]) -> Vec<u8> {
        use snap::read;
        use std::io::Read;
    
        let mut buf = vec![];
        read::FrameDecoder::new(bytes).read_to_end(&mut buf).unwrap();
        buf
    }
}

#[cfg(test)]
mod test {
    use crate::compress::compress::Compress;
    use super::*;
    
    #[test]
    fn basics() {
        let data = "fsfjlahuhdwnf.v.sljp;jdqdsjdfhalkshdlhliqjdna,dnlawjdla.jdj.lskd.wnkak".as_bytes();
        let compress = Snappy::new();
        let compressed = compress.encode(&data);
        compress.decode(&compressed);
    }
}