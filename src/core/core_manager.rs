use crate::buf;
use crate::core::bit;
use crate::core::pit;
use crate::core::vam;

pub struct CoreManager {
    bit: bit::BIT,
    pit: pit::PIT,
    vam: vam::VAM,
    buf_cache: buf::BufCache,
}

impl CoreManager {
    pub fn new() -> CoreManager {
        CoreManager {
            bit: bit::BIT::new(),
            pit: pit::PIT::new(),
            vam: vam::VAM::new(),
            buf_cache: buf::BufCache::new(),
        }
    }

    pub fn read_sb(&mut self) {

    }

    pub fn mount() {

    }



}

// 管理BIT Region
impl CoreManager {
    pub fn read_bit(&mut self) {
        let data_1 = self.read_block(1);
        let data_2 = self.read_block(2);
    }
}

// 管理PIT Region
impl CoreManager {
    pub fn read_pit(&mut self) {
        
    }
}

// 调用下层的接口 对上不可见
impl CoreManager {
    pub fn read_page(&self, address: u32) -> [u8; 4096] {
        self.buf_cache.read(0, address)
    }

    pub fn read_block(&self, block_no: u32) -> [[u8; 4096]; 128] {
        let max_address = block_no * 128 + 1;
        let mut address = max_address - 128;
        let mut block = vec![];
        while address < max_address {
            let page = self.read_page(address);
            address += 1;
            block.push(page);
        }
        block.try_into().unwrap()
    }

    pub fn write_page(&self, address: u32, data: [u8; 4096]) -> bool {
        self.buf_cache.write(0, address, data)
    }

    pub fn write_block(&self, block_no: u32, data: [[u8; 4096]; 128]) {
        let mut address = block_no * 128 - 127;
        for data in data.into_iter() {
            self.write_page(address, data);
            address += 1;
        }
    }
}

// 对上层提供的接口
impl CoreManager {
    pub fn read_data(&self, v_address: u32) -> [u8; 4096] {
        let address = self.vam.get_physic_address(v_address).unwrap();
        self.read_page(address)
    }
}
