
use crate::buf;





pub struct SSTableMetadata<'a> {
    metablock: Vec<usize>,
    bufcache: &'a buf::BufCache,
    max_length: usize,
}

impl SSTableMetadata<'a> {
    pub fn new(bufcache: &'a buf::BufCache, max_length: usize) -> 'a SSTableMetadata {
        SSTableMetadata {
            metablock: Vec<usize>::new(),
            bufcache: bufcache,
            max_length: max_length,
        }
    }

    pub fn read(offset: usize) -> Vec<u8> {
        
    }

    pub fn write(offset: usize, length: usize, data: Vec<u8>) {
    
    } 
}