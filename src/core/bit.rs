use std::collections::HashMap;

pub struct BIT {
    pub table: HashMap<u32, bool>, // true: dirty/used false: clean
    pub sync: bool,                // true 需要持久化到磁盘中
}

impl BIT {
    pub fn new() -> BIT {
        BIT {
            table: HashMap::new(),
            sync: false,
        }
    }

    pub fn get_page(&self, address: u32) -> bool {
        self.table.get(&address).unwrap().clone()
    }

    pub fn set_page(&mut self, address: u32, status: bool) -> bool {
        *self.table.get_mut(&address).unwrap() = status;
        self.sync = true;
        true
    }
    
    pub fn get_block(&self, block_no: u32) -> Option<[bool; 128]> {
        let mut res = [false; 128];
        let start_index = block_no * 128;
        let end_index = (block_no + 1) * 128;
        for (index, i) in (start_index..end_index).enumerate() {
            res[index] = self.table.get(&i).unwrap().clone();
        }
        Some(res)
    }

    pub fn set_block(&mut self, block_no: u32, status: [bool; 128]) -> bool {
        let start_index = block_no * 128;
        let end_index = (block_no + 1) * 128;
        for (index, i) in (start_index..end_index).enumerate() {
            *self.table.get_mut(&i).unwrap() = status[index];
        }
        self.sync = true;
        true
    }

    pub fn encode(&self) -> [[u8; 4096]; 128] {
        todo!()
    }

    pub fn need_sync(&self) -> bool {
        self.sync
    }

    pub fn sync(&mut self) {
        self.sync = false;
    }
}

