use std::collections::HashMap;

pub struct PIT {
    pub table: HashMap<u32, u32>,  // page -> ino
    pub sync: bool,                // true 需要持久化到磁盘中
}

impl PIT {
    pub fn new() -> PIT {
        PIT {
            table: HashMap::new(),
            sync: false,
        }
    }

    pub fn set_by_disk(&mut self, data: [[[u8; 4096]; 128]; 32]) {
        let iter = DataRegion::new(data);
        for (index, ino) in iter.enumerate() {
            self.table.insert(index as u32, ino);
        }
    }

    pub fn get_page(&self, address: u32) -> u32 {
        self.table.get(&address).unwrap().clone()
    }

    pub fn set_page(&mut self, address: u32, status: u32) -> bool {
        *self.table.get_mut(&address).unwrap() = status;
        self.sync = true;
        true
    }

    pub fn encode(&self) -> [[[u8; 4096]; 128]; 32] {
        todo!()
    }

    pub fn need_sync(&self) -> bool {
        self.sync
    }

    pub fn sync(&mut self) {
        self.sync = false;
    }

}

struct DataRegion {
    count: usize,
    data: Vec<u8>,
}

impl DataRegion {
    fn new(data: [[[u8; 4096]; 128]; 32]) -> DataRegion {
        let mut arr = vec![];
        for block in data.iter() {
            for page in block.iter() {
                for byte in page.iter() {
                    arr.push(byte.clone());
                }
            }
        }
        DataRegion {
            count: 0,
            data: arr,
        }
    }
}

impl Iterator for DataRegion {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.data.len() {
            return None;
        }
        let byte_1 = self.data.get(self.count).unwrap();
        let byte_2 = self.data.get(self.count+1).unwrap();
        let byte_3 = self.data.get(self.count+2).unwrap();
        let byte_4 = self.data.get(self.count+3).unwrap();
        self.count += 4;
        Some((byte_1 << 24 + byte_2 << 16 + byte_3 << 8 + byte_4) as u32)
    }
}