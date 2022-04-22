use crate::gc::main_table;

#[derive(Clone, Copy)]
pub enum PageUsedStatus {
    Clean,
    Dirty,
    Busy(u32),
}

pub struct GCManager {
    table: main_table::MainTable,
}

impl GCManager {
    pub fn new() -> GCManager {
        GCManager {
            table: main_table::MainTable::new(),
        }
    }
}

// 提供MainTable的接口
impl GCManager {
    pub fn set_table(&mut self, address: u32, status: PageUsedStatus) {
        self.table.set_page(address, status);
    }

    pub fn get_table(&mut self, address: u32) -> PageUsedStatus {
        self.table.get_page(address)
    }
}