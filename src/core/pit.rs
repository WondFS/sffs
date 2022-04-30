use std::collections::HashMap;
use crate::util::array;

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

    pub fn init_page(&mut self, address: u32, status: u32) {
        if self.table.contains_key(&address) {
            panic!("PIT: init page has exist");
        }
        self.table.insert(address, status);
    }

    pub fn get_page(&self, address: u32) -> u32 {
        if !self.table.contains_key(&address) {
            panic!("PIT: get not that page");
        }
        self.table.get(&address).unwrap().clone()
    }

    pub fn set_page(&mut self, address: u32, status: u32) {
        if !self.table.contains_key(&address) {
            panic!("PIT: set not that page");
        }
        *self.table.get_mut(&address).unwrap() = status;
        self.sync = true;
    }

    pub fn delete_page(&mut self, address: u32) {
        if !self.table.contains_key(&address) {
            panic!("PIT: delete not that page");
        }
        self.table.remove(&address).unwrap();
    }

    pub fn encode(&self) -> [[u8; 4096]; 128] {
        let mut res: [u32; 128 * 4096 / 4] = [0; 128 * 4096 / 4];
        for (key, value) in &self.table {
            res[*key as usize] = *value;
        }
        let mut temp: [u8; 128 * 4096] = [0; 128 * 4096];
        for (index, value) in res.iter().enumerate() {
            let byte_1 = (*value >> 24) as u8;
            let byte_2 = (*value >> 16) as u8;
            let byte_3 = (*value >> 8) as u8;
            let byte_4 = (*value >> 8) as u8;
            let start_index = index * 4;
            temp[start_index] = byte_1;
            temp[start_index + 1] = byte_2;
            temp[start_index + 2] = byte_3;
            temp[start_index + 3] = byte_4;
        }
        let mut data: [[u8; 4096]; 128] = [[0; 4096]; 128];
        for (index, value) in temp.iter().enumerate() {
            let i = index / 128;
            let j = index % 128;
            data[i][j] = *value;
        }
        data
    }

    pub fn aaa(&self) -> Vec<u8> {
        let mut res: [u32; 128 * 4096 / 4] = [0; 128 * 4096 / 4];
        for (key, value) in &self.table {
            res[*key as usize] = *value;
        }
        let mut temp: Vec<u8> = vec![];
        for value in res.iter() {
            let byte_1 = (*value >> 24) as u8;
            let byte_2 = (*value >> 16) as u8;
            let byte_3 = (*value >> 8) as u8;
            let byte_4 = (*value >> 8) as u8;
            temp.push(byte_1);
            temp.push(byte_2);
            temp.push(byte_3);
            temp.push(byte_4);
        }
        temp
    }

    pub fn ecode_util(temp: Vec<u8>) {
        let mut data: [[u8; 4096]; 128] = [[0; 4096]; 128];
        for (index, value) in temp.into_iter().enumerate() {
            let i = index / 4096;
            let j = index % 4096;
            data[i][j] = value;
        }
    }

    pub fn need_sync(&self) -> bool {
        self.sync
    }

    pub fn sync(&mut self) {
        self.sync = false;
    }

}

pub struct DataRegion {
    count: usize,
    data: Vec<u8>,
}

// 按原设计的32会爆栈
impl DataRegion {
    pub fn new(data: [[u8; 4096]; 128]) -> DataRegion {
        let mut arr = vec![];
        for page in data.iter() {
            for byte in page.iter() {
                arr.push(byte.clone());
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
        if self.count < self.data.len() {
            let byte_1 = (*self.data.get(self.count).unwrap() as u32) << 24;
            let byte_2 = (*self.data.get(self.count+1).unwrap() as u32) << 16;
            let byte_3 = (*self.data.get(self.count+2).unwrap() as u32) << 8;
            let byte_4 = *self.data.get(self.count+3).unwrap() as u32;
            self.count += 4;
            Some(byte_1 + byte_2 + byte_3 + byte_4)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        // let mut pit = PIT::new();
        // let data = [[0; 4096]; 128];
        // let iter = DataRegion::new(data);
        // for (index, ino) in iter.enumerate() {
            // if ino != 0 {
                // pit.init_page(index as u32, ino);
            // }
        // }
        // let mut data: [[u8; 4096]; 128] = [[0; 4096]; 128];
        // let temp = pit.aaa();
        // let mut data: [[u8; 4096]; 128] = [[0; 4096]; 128];
        // PIT::ecode_util(temp);
        // assert_eq!(pit.encode(), data);
        // assert_eq!(pit.need_sync(), false);

        // let mut data: [[u8; 4096]; 128] = [[0; 4096]; 128];
        // for (index, value) in temp.iter().enumerate() {
        //     let i = index / 4096;
        //     let j = index % 4096;
        //     data[i][j] = *value;
        // }

        // let mut data_1: [[u8; 4096]; 64] = [[0; 4096]; 64];
        // let mut data_2: [[u8; 4096]; 64] = [[0; 4096]; 64];
        // let mut data: [[u8; 4096]; 128] = [[0; 4096]; 128];
        // for (index, value) in temp.into_iter().enumerate() {
        //     let i = index / 4096;
        //     let j = index % 4096;
        //     data[i][j] = value;
        // }
    }

    // #[test]
    // fn basics() {


    // }
}
