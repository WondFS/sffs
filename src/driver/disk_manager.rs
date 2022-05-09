use crate::write_buf;
use crate::util::array;
use crate::driver::{disk, fake_disk};

pub struct DiskManager {
    pub is_virtual: bool,
    pub driver: Option<disk::DiskDriver>,
    pub fake_disk: Option<fake_disk::FakeDisk>,
    pub write_cache: write_buf::WriteCache,
}

impl DiskManager {
    pub fn new(is_virtual: bool) -> DiskManager {
        let mut driver = None;
        let mut fake_disk = None;
        if is_virtual {
            fake_disk = Some(fake_disk::FakeDisk::new(4096)); // 32 Block 1024 Page
        } else {
            driver = Some(disk::DiskDriver::new());
        }
        DiskManager {
            is_virtual,
            driver,
            fake_disk,
            write_cache: write_buf::WriteCache::new(),
        }
    }

    pub fn read(&self, block_no: u32) -> [[u8; 4096]; 128] {
        let start_index = block_no * 128;
        let end_index = (block_no + 1) * 128;
        let mut exist_indexs = vec![];
        for index in start_index..end_index {
            if self.write_cache.contains_address(index) {
                exist_indexs.push(index);
            }
        }
        let mut block_data = DiskManager::transfer(self.disk_read(block_no));
        for index in exist_indexs.into_iter() {
            let data = self.write_cache.read(index).unwrap();
            block_data.set(index - start_index, data);
        }
        DiskManager::reverse(&block_data)
    }

    pub fn disk_read(&self, block_no: u32) -> [[u8; 4096]; 128] {
        if self.is_virtual {
            self.fake_disk.as_ref().unwrap().fake_disk_read(block_no)
        } else {
            self.driver.as_ref().unwrap().disk_read(block_no)
        }
    }
    
    pub fn disk_write(&mut self, address: u32, data: [u8; 4096]) {
        self.write_cache.write(address, data);
        if !self.write_cache.need_sync() {
            return;
        }
        let data = self.write_cache.get_all();
        for entry in data.into_iter() {
            if self.is_virtual {
                return self.fake_disk.as_mut().unwrap().fake_disk_write(entry.0, entry.1);
            }
            self.driver.as_mut().unwrap().disk_write(entry.0, entry.1);
        }
        self.write_cache.sync();
    }
    
    pub fn disk_erase(&mut self, block_no: u32) {
        let start_index = block_no * 128;
        let end_index = (block_no + 1) * 128;
        for index in start_index..end_index {
            self.write_cache.recall_write(index);
        }
        if self.is_virtual {
            self.fake_disk.as_mut().unwrap().fake_disk_erase(block_no);
            return;
        }
        self.driver.as_mut().unwrap().disk_erase(block_no);
    }

    pub fn transfer(data: [[u8; 4096]; 128]) -> array::Array1<[u8; 4096]> {
        let mut res = array::Array1::new(128);
        res.init([0; 4096]);
        for i in 0..128 {
            res.set(i, data[i as usize]);
        }
        res
    }

    pub fn reverse(data: &array::Array1<[u8; 4096]>) -> [[u8; 4096]; 128] {
        let mut ret = [[0; 4096]; 128];
        for i in 0..128 {
            ret[i] = data.get(i as u32);
        }
        ret
    }
}

mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut manager = DiskManager::new(true);

        let data = [1; 4096];
        manager.disk_write(100, data);
        let data = manager.read(0);
        assert_eq!(data[100], [1; 4096]);

        manager.disk_erase(0);
        let data = manager.read(0); 
        assert_eq!(data[100], [0; 4096]);
    }
}