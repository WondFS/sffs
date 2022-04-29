use crate::gc::gc_event;
use crate::gc::main_table;
use crate::gc::block_table;

#[derive(Clone, Copy, PartialEq, Debug)]
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
            block_table: block_table::BlockTable::new(32),
        }
    }

    pub fn find_next_pos_to_write(&self, size: u32) -> Option<u32> {
        for block in self.block_table.table.iter() {
            if block.reserved_size >= size {
                let offset = block.reserved_offset;
                return Some((block.block_no * 128) as u32 + offset);
            }
        }
        None
    }

    pub fn find_next_pos_to_write_except(&self, size: u32, block_no: u32) -> Option<u32> {
        for block in self.block_table.table.iter() {
            if block.reserved_size >= size && block.block_no != block_no {
                let offset = block.reserved_offset;
                return Some((block.block_no * 128) as u32 + offset);
            }
        }
        None
    }

    pub fn generate_gc_event(&mut self) -> gc_event::GCEventGroup {
        let mut gc_block = self.block_table.table[0];
        for block in self.block_table.table.iter() {
            if block.reserved_size < gc_block.reserved_size {
                gc_block = *block;
            }
        }
        let mut used_entries: Vec<(u32, u32, u32, u32)> = vec![];
        let block_no = gc_block.block_no;
        let start_index = block_no * 128;
        let end_index = (block_no + 1) * 128;
        let mut size = 0;
        let mut last_entry: Option<(u32, u32, u32, u32)> = None;
        for address in start_index..end_index {
            let status = self.main_table.get_page(address);
            match status {
                PageUsedStatus::Busy(ino) => {
                    if last_entry.is_some() {
                        if last_entry.unwrap().0 == ino {
                            size += 1;
                        } else {
                            last_entry.unwrap().1 = size;
                            used_entries.push(last_entry.unwrap());
                            last_entry = Some((ino, 0, address, 0));
                        }
                    } else {
                        last_entry = Some((ino, 0, address, 0));
                        size = 1;
                    }
                }
                _ => {
                    if last_entry.is_some() {
                        last_entry.as_mut().unwrap().1 = size;
                        used_entries.push(last_entry.unwrap());
                        last_entry = None;
                        size = 0;
                    }
                }
            }
        }
        if last_entry.is_some() {
            last_entry.unwrap().1 = size;
            used_entries.push(last_entry.unwrap());
        }
        for entry in used_entries.iter_mut() {
            let d_address = self.find_next_pos_to_write_except(entry.1, block_no);
            entry.3 = d_address.unwrap();
        }
        let mut gc_group = gc_event::GCEventGroup::new();
        let mut index = 0;
        for entry in used_entries {
            let event = gc_event::MoveGCEvent {
                index,
                ino: entry.0,
                size: entry.1,
                o_address: entry.2,
                d_address: entry.3,
            };
            gc_group.events.push(gc_event::GCEvent::Move(event));
            index += 1;
        }
        let event = gc_event::EraseGCEvent {
            index,
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

    pub fn get_table(&self, address: u32) -> PageUsedStatus {
        self.main_table.get_page(address)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut manager = GCManager::new();

        for address in 0..32 * 128 {
            manager.set_table(address, PageUsedStatus::Clean);
        }

        assert_eq!(manager.find_next_pos_to_write(5), Some(0));
        
        manager.set_table(0, PageUsedStatus::Busy(0));
        manager.set_table(1, PageUsedStatus::Busy(0));
        manager.set_table(2, PageUsedStatus::Busy(0));
        manager.set_table(3, PageUsedStatus::Busy(0));
        manager.set_table(4, PageUsedStatus::Busy(0));

        assert_eq!(manager.get_table(0), PageUsedStatus::Busy(0));
        assert_eq!(manager.find_next_pos_to_write(128), Some(128));

        let event = manager.generate_gc_event();
        assert_eq!(event.events[0], gc_event::GCEvent::Move(gc_event::MoveGCEvent{ index: 0, ino: 0, size: 5, o_address: 0, d_address: 128 }));
        assert_eq!(event.events[1], gc_event::GCEvent::Erase(gc_event::EraseGCEvent{ index: 1, block_no: 0 }));
    }
}