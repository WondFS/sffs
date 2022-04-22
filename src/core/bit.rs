use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum PageUsedStatus {
    Clean,
    Dirty,
    Busy(u32),
}

pub struct BIT {
    pub table: HashMap<u32, PageUsedStatus>,
    pub need_sync: bool, // true 需要持久化到磁盘中
}

impl BIT {
    pub fn new() -> BIT {
        BIT {
            table: HashMap::new(),
            need_sync: false,
        }
    }

    pub fn set_by_disk(&mut self) {
        
    }

    pub fn get_page(&self, address: u32) -> PageUsedStatus {
        todo!()
    }

    pub fn set_page(&mut self, address: u32, status: PageUsedStatus) {

    }

    pub fn update_page(&mut self, address: u32, status: PageUsedStatus) {
        
    }

    pub fn get_block(&self, block_no: u32) -> Option<[PageUsedStatus; 128]> {
        let mut res = [PageUsedStatus::Clean; 128];
        let start_index = block_no * 128;
        let end_index = (block_no + 1) * 128;
        for (index, i) in (start_index..end_index).enumerate() {
            res[index] = self.table.get(&i).unwrap().clone();
        }
        Some(res)
    }

    pub fn update_block(&mut self, block_no: u32, status: [PageUsedStatus; 128]) {

    }

    pub fn find_next_pos_to_write(&self, len: u32) -> Option<u32> {
        todo!()
    }

    pub fn find_garbage_collect_event(&self) {
        todo!()
    }

    pub fn sync(&mut self) {
        
    }
}

