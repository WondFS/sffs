#[allow(dead_code)]
pub struct Buf {
    pub valid: bool,
    pub disk: bool,
    pub dev: u8,
    pub address: u32,
    pub ref_cnt: u8,
    pub data: [u8; 4096],
}

#[allow(dead_code)]
pub struct BufCache {

}

#[allow(dead_code)]
#[allow(unused_variables)]
impl BufCache {
    pub fn new() -> BufCache {
        BufCache {  }
    }   

    pub fn read(&self, dev: u8, address: u32) -> [u8; 4096] {
        [0; 4096]
    }

    pub fn write(&self, dev: u8, address: u32, data: [u8; 4096]) -> bool {
        true
    }
}