use crate::gc::gc_event;
use crate::gc::main_table;
use crate::gc::block_table;

#[derive(Clone, Copy)]
pub enum PageUsedStatus {
    Clean,
    Dirty,
    Busy(u32),
}

pub struct GCManager {
    main_table: main_table::MainTable,
    block_table: block_table::BlockTable,
}

impl GCManager {
    pub fn new() -> GCManager {
        GCManager {
            main_table: main_table::MainTable::new(),
            block_table: block_table::BlockTable::new(4096),
        }
    }

    pub fn find_next_pos_to_write(&mut self, size: u32) -> Option<u32> {
        for block in self.block_table.table.iter() {
            if block.reserved_size > size {
                return Some(block.reserved_offset);
            }
        }
        None
    }

    pub fn generate_gc_event(&mut self) -> gc_event::GCEventGroup {
        let mut gc_block = self.block_table.table[0];
        for block in self.block_table.table.iter() {
            if block.reserved_size > gc_block.reserved_size {
                gc_block = block.clone();
            }
        }
        let mut used_entries: Vec<(u32, u32, i32)> = vec![];
        let block_no = gc_block.block_no;
        let start_index = block_no * 128;
        let end_index = (block_no + 1) * 128;
        let mut size = 0;
        let mut last_ino = 0;
        for address in start_index..end_index {
            let status = self.main_table.get_page(address);
            match status {
                PageUsedStatus::Busy(ino) => {
                    let last = used_entries.last();
                    if last.is_some() {
                        last_ino = last.unwrap().0;
                        if last_ino == ino {
                            
                        }
                    }

                    used_entries.push((address, ino, size));
                    size += 1;
                }
                _ => {
                    size = 0;
                }
            }
        }
        let mut gc_group = gc_event::GCEventGroup::new();
        for used_entry in used_entries {

        }
        let event = gc_event::EraseGCEvent {
            index: 0,
            block_no,
        };
        gc_group.events.push(gc_event::GCEvent::Erase(event));
        gc_group
    }
}

// 提供MainTable的接口
impl GCManager {
    pub fn set_table(&mut self, address: u32, status: PageUsedStatus) {
        match status {
            PageUsedStatus::Busy(_) => {
                self.block_table.use_page(address);
            }
            PageUsedStatus::Clean => {
                self.block_table.clean_page(address);
            }
            _ => ()
        }
        self.main_table.set_page(address, status);
    }

    pub fn get_table(&mut self, address: u32) -> PageUsedStatus {
        self.main_table.get_page(address)
    }
}