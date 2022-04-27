use crate::util::lru_cache;

#[derive(Clone, Copy)]
pub struct Buf {
    pub valid: bool,
    pub disk: bool,
    pub dev: u8,
    pub address: u32,
    pub ref_cnt: u8,
    pub data: [u8; 4096],
}

impl Buf {
    pub fn new() -> Buf {
        todo!()
    }
}

pub struct BufCache {
    pub size: usize,
    pub capacity: usize,
    pub cache: lru_cache::LRUCache<Buf>,
}

impl BufCache {
    pub fn new() -> BufCache {
        let capacity = 4096;
        BufCache {
            size: 0,
            capacity: capacity as usize,
            cache: lru_cache::LRUCache::new(capacity as usize),
        }
    }   

    pub fn read(&self, dev: u8, address: u32) -> [u8; 4096] {
        [0; 4096]
    }

    pub fn write(&self, dev: u8, address: u32, data: [u8; 4096]) {
        todo!()
    }

    pub fn erase(&self, dev: u8, block_no: u32) {
        todo!()
    }
}