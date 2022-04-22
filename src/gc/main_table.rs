use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum PageUsedStatus {
    Clean,
    Dirty,
    Busy(u32),
}

// 内存中的disk全局信息
pub struct MainTable {
    pub table: HashMap<u32, PageUsedStatus>,
}

impl MainTable {
    fn new() -> MainTable {
        MainTable {
            table: HashMap::new(),
        }
    }
}