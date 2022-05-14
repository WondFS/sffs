
use crate::buf;
use crate::kv::lsm_tree::kv;
use crate::kv::lsm_tree::data_type;
use std::sync::Arc;
use std::cell::RefCell;
use crate::kv::lsm_tree::data_type::{Key, Value, Entry};










pub struct BlockDataIterator {
    block_id: u32,
    key: Vec<Key>,
    value: Vec<Value>,
    iter: u32,
    entry_num: u32,
    EOF_KEY: Key,
    EOF_VALUE: Value,
}

impl BlockDataIterator {
    pub fn new(block_id: u32, buf: Arc<RefCell<buf::BufCache>>, max_entries_num: u32) -> BlockDataIterator {
        let data = buf.borrow_mut().read(0, block_id);
        
        let mut key: Vec<Key> = Vec::new();
        let mut value: Vec<Value> = Vec::new();

        let EOF_KEY = data_type::fill_str(data_type::EOF, data_type::KEY_LENGTH as usize);
        let EOF_VALUE = data_type::fill_str(data_type::EOF, data_type::VALUE_LENGTH as usize);

        let mut entry_num: u32 = max_entries_num;

        for i in 0..kv::entry_num_per_block {
            let key_s = i * data_type::ENTRY_LENGTH as u32;
            let key_t = i * data_type::ENTRY_LENGTH as u32 + data_type::KEY_LENGTH as u32;

            let value_s = i * data_type::ENTRY_LENGTH as u32 + data_type::KEY_LENGTH as u32;
            let value_t = i * data_type::ENTRY_LENGTH as u32 + data_type::KEY_LENGTH as u32 + data_type::VALUE_LENGTH;

            let k = data[key_s as usize..key_t as usize].to_vec();
            let v = data[value_s as usize..value_t as usize].to_vec();

            if k == EOF_KEY && v == EOF_VALUE {
                entry_num = i;
                break;
            } else {
                key.push(k);
                value.push(v);
            }
        }

        BlockDataIterator {
            block_id: block_id,
            key: key,
            value: value,
            iter: 0,
            entry_num: entry_num,
            EOF_KEY: EOF_KEY,
            EOF_VALUE: EOF_VALUE,
        }
    }


    pub fn hasNext(&self) -> bool {
        self.iter < self.entry_num
    }

    pub fn Next(&mut self) -> Entry {
        let ret = self.iter;
        self.iter += 1;

        Entry::new(self.key.get(ret as usize).unwrap().clone(), self.value.get(ret as usize).unwrap().clone())
    }


}