use crate::buf;
use crate::kv::lsm_tree::data_type;
use crate::kv::lsm_tree::sstable_metadata;
use std::str;
use crate::kv::lsm_tree::memtable;







pub struct LSMTree {
    memtable: memtable::Memtable,
    sstable_metadata: sstable_metadata::SSTableMetadata,
}

pub static sstableNumThreshold: u32 = 10;
pub static entry_num_per_block: u32 = 4096 as u32 / data_type::ENTRY_LENGTH as u32;
pub static memtableThreshold: u32 = entry_num_per_block - 1;

impl LSMTree {
    pub fn new() -> LSMTree {
        LSMTree {
            memtable: memtable::Memtable::new(memtableThreshold),
            sstable_metadata: sstable_metadata::SSTableMetadata::new(sstableNumThreshold),
        }
    }

    pub fn flush(&mut self) -> bool {
    
        self.sstable_metadata.flush(self.memtable.flush().clone());

        true
    }


    pub fn put(&mut self, key_str: &str, value_str: &str) -> bool {
        if key_str.as_bytes().to_vec().len() > data_type::KEY_LENGTH as usize {
            return false;
        }

        if value_str.as_bytes().to_vec().len() > data_type::VALUE_LENGTH as usize {
            return false;
        }


        let key = data_type::fill_str(key_str, data_type::KEY_LENGTH as usize);
        let value = data_type::fill_str(value_str, data_type::VALUE_LENGTH as usize);

        self.memtable.put(&key, &value);

        if self.memtable.full() == false {
            return true;
        }

        self.flush();

        if self.sstable_metadata.full() {
            self.sstable_metadata.merge();
        }

        true
    }

    pub fn get(&mut self, key_str: &str) -> Option<String> {
        if key_str.as_bytes().to_vec().len() > data_type::KEY_LENGTH as usize {
            return None;
        }

        let key = data_type::fill_str(key_str, data_type::KEY_LENGTH as usize);
        let res: String;



        match self.memtable.get(&key) {
            Some(v) => {
                res = data_type::vec_to_str(&v);
                if res != data_type::TOMBSTONE.to_string() {
                    return Some(res);
                } else {
                    return None;
                }
            }

            _ => {
                return self.sstable_metadata.get(key);
            }
        }
    }

    

    pub fn delete(&mut self, key_str: &str) -> bool {
        if key_str.as_bytes().to_vec().len() > data_type::KEY_LENGTH as usize {
            return false;
        }
        
        let key = data_type::fill_str(key_str, data_type::KEY_LENGTH as usize);
        let value = data_type::fill_str(data_type::TOMBSTONE, data_type::VALUE_LENGTH as usize);

        self.memtable.put(&key, &value);

        true
    }
}