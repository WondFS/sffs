use std::collections::HashMap;
use crate::gc::gc_manager;

// 内存中的disk全局信息
pub struct MainTable {
    pub table: HashMap<u32, gc_manager::PageUsedStatus>,
}

impl MainTable {
    pub fn new() -> MainTable {
        MainTable {
            table: HashMap::new(),
        }
    }

    pub fn set_page(&mut self, address: u32, status: gc_manager::PageUsedStatus) {
        *self.table.get_mut(&address).unwrap() = status;
    }

    pub fn get_page(&mut self, address: u32) -> gc_manager::PageUsedStatus {
        self.table.get(&address).unwrap().clone()
    }
    
}