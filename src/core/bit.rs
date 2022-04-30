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

    pub fn init_page(&mut self, address: u32, status: bool) {
        if self.table.contains_key(&address) {
            panic!("BIT: init page has exist");
        }
        self.table.insert(address, status);
    }

    pub fn get_page(&self, address: u32) -> bool {
        if !self.table.contains_key(&address) {
            panic!("BIT: get not that page");
        }
        self.table.get(&address).unwrap().clone()
    }

    pub fn set_page(&mut self, address: u32, status: bool) {
        if !self.table.contains_key(&address) {
            panic!("BIT: set not that page");
        }
        *self.table.get_mut(&address).unwrap() = status;
        self.sync = true;
    }
    
    pub fn get_block(&self, block_no: u32) -> Option<[bool; 128]> {
        let mut res = [false; 128];
        let start_index = block_no * 128;
        let end_index = (block_no + 1) * 128;
        for (index, i) in (start_index..end_index).enumerate() {
            res[index] = self.get_page(i);
        }
        Some(res)
    }

    pub fn set_block(&mut self, block_no: u32, status: [bool; 128]) {
        let start_index = block_no * 128;
        let end_index = (block_no + 1) * 128;
        for (index, i) in (start_index..end_index).enumerate() {
            self.set_page(i, status[index]);
        }
    }

    pub fn encode(&self) -> [[u8; 4096]; 128] {
        let mut res: [u8; 128 * 4096] = [0; 128 * 4096];
        for (key, value) in &self.table {
            let index = key / 8;
            let off = key % 8;
            if *value {
                res[index as usize] = res[index as usize] | 1 << off;
            }
        }
        let mut data: [[u8; 4096]; 128] = [[0; 4096]; 128];
        for (index, temp) in res.iter().enumerate() {
            let i = index / 4096;
            let j = index % 4096;
            data[i][j] = *temp;
        }
        data
    }

    pub fn need_sync(&self) -> bool {
        self.sync
    }

    pub fn sync(&mut self) {
        self.sync = false;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn basics() {
        let mut bit = BIT::new();
        let mut data: [[u8; 4096]; 128] = [[0; 4096]; 128];
        data[100][3] = 3;
        data[10][2] = 68;
        for (i, page) in data.iter().enumerate() {
            for (j, byte) in page.iter().enumerate() {
                let mut byte = byte.clone();
                for k in 0..8 {
                    let index = i * 4096 * 8 + j * 8 + k;
                    if byte & 1 == 1 {
                        bit.init_page(index as u32, true);
                    } else {
                        bit.init_page(index as u32, false);
                    }
                    byte = byte >> 1;
                }
            }
        }
        assert_eq!(bit.encode(), data);
        assert_eq!(bit.need_sync(), false);
        bit.set_page(200, true);
        assert_eq!(bit.get_page(200), true);
        assert_eq!(bit.need_sync(), true);
        let data = [true; 128];
        bit.set_block(10, data);
        assert_eq!(bit.get_block(10).unwrap(), data);
    }
}

