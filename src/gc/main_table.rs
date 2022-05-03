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
        if !self.table.contains_key(&address) {
            self.table.insert(address, status);
            return;
        }
        *self.table.get_mut(&address).unwrap() = status;
    }

    pub fn get_page(&self, address: u32) -> gc_manager::PageUsedStatus {
        if !self.table.contains_key(&address) {
            panic!("MainTable: get no that page");
        }
        self.table.get(&address).unwrap().clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut tabel = MainTable::new();

        tabel.set_page(100, gc_manager::PageUsedStatus::Clean);
        tabel.set_page(101, gc_manager::PageUsedStatus::Dirty);
        tabel.set_page(102, gc_manager::PageUsedStatus::Busy(20));
        tabel.set_page(103, gc_manager::PageUsedStatus::Busy(22));
        
        assert_eq!(tabel.get_page(100), gc_manager::PageUsedStatus::Clean);
        assert_eq!(tabel.get_page(101), gc_manager::PageUsedStatus::Dirty);
        assert_eq!(tabel.get_page(102), gc_manager::PageUsedStatus::Busy(20));
        assert_eq!(tabel.get_page(103), gc_manager::PageUsedStatus::Busy(22));

        tabel.set_page(102, gc_manager::PageUsedStatus::Busy(21));
        assert_eq!(tabel.get_page(102), gc_manager::PageUsedStatus::Busy(21));
    }
}