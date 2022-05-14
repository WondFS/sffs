
use std::convert::TryInto;
use std::result::Result;
use crate::buf;
use std::collections::HashMap;
use std::sync::Arc;
use std::cmp::Ordering;
use std::cell::RefCell;
use crate::kv::lsm_tree::file_iterator;
use crate::kv::lsm_tree::data_type;


pub type bufLink = Arc<RefCell<buf::BufCache>>;


pub struct SSTableMetadata {
    metablock: HashMap<u32, Vec<u32>>,
    bufcache: bufLink,
    cur_block_id: u32,
    sstable_max_id: u32,
    pub sstable_num: u32,
    pub sstable_num_threshold: u32,
    entry_num_per_block: u32,
    EOF_KEY: data_type::Key,
    EOF_VALUE: data_type::Value,
}

impl SSTableMetadata {
    pub fn new(sstable_num_threshold: u32) -> SSTableMetadata {
        SSTableMetadata {
            metablock: HashMap::new(),
            bufcache: Arc::new(RefCell::new(buf::BufCache::new())),
            cur_block_id: 0,
            sstable_max_id: 0,
            sstable_num: 0,
            sstable_num_threshold: sstable_num_threshold,
            entry_num_per_block: 4096 as u32 / data_type::ENTRY_LENGTH as u32,
            EOF_KEY: data_type::fill_str(data_type::EOF, data_type::KEY_LENGTH as usize),
            EOF_VALUE: data_type::fill_str(data_type::EOF, data_type::VALUE_LENGTH as usize),
        }
    }

    pub fn trans<T, const N: usize>(&self, v: Vec<T>) -> [T; N] {
        v.try_into()
            .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length{} but it was {}", 4096, v.len()))
    }

    pub fn full(&self) -> bool {
        self.sstable_num > self.sstable_num_threshold
    }

    pub fn get(&self, key: data_type::Key) -> Option<String> {
        
        let num = self.sstable_num;

        for i in 0..num {
            let index = self.sstable_max_id - i;
            let blocks = self.metablock.get(&index).unwrap();
            if let Some(val) = file_iterator::fileIterator::new(
                                                    blocks.clone(), self.bufcache.clone(), self.entry_num_per_block)
                                                    .get(&key) {
                return Some(data_type::vec_to_str(&val));
            }
        }

        None
    }

    pub fn flush(&mut self, mut memtable_entries: Vec<data_type::Entry>) -> bool {
        self.sstable_max_id += 1;
        self.sstable_num += 1;
        self.metablock.insert(self.sstable_max_id, Vec::new());

        let mut cnt = 0;
        let mut block_data: Vec<u8> = Vec::new();

        for entry in memtable_entries.iter_mut() {

            cnt += 1;
            block_data.append(&mut entry.key);
            block_data.append(&mut entry.value);
            
            if cnt == self.entry_num_per_block - 1 {
                self.cur_block_id += 1;
                
                self.metablock.get_mut(&self.sstable_max_id).unwrap().push(self.cur_block_id);

                block_data.append(&mut self.EOF_KEY.clone());
                block_data.append(&mut self.EOF_VALUE.clone());

                self.bufcache.borrow_mut().write(0, self.cur_block_id, self.trans(block_data.clone()));

                block_data.clear();
            }
        }

        if cnt != 0 {
            panic!("LSM_TREE: entries should be multitimes block size");
        }

        true
    }

    pub fn merge(&mut self) {
        let A = self.sstable_max_id - self.sstable_num + 1;
        let B = A + 1;

        let aBlock = self.metablock.get(&A).unwrap();
        let bBlock = self.metablock.get(&B).unwrap();
        
        let mut aData = file_iterator::fileIterator::new(aBlock.clone(), self.bufcache.clone(), self.entry_num_per_block);
        let mut bData = file_iterator::fileIterator::new(bBlock.clone(), self.bufcache.clone(), self.entry_num_per_block);

        let mut newBlock: Vec<u32> = Vec::new();
        let mut block_data: Vec<u8> = Vec::new();
        let mut cnt: u32 = 0;

        let mut aKey: Option<data_type::Key> = None;
        let mut bKey: Option<data_type::Key> = None;
        let mut aValue: Option<data_type::Value> = None;
        let mut bValue: Option<data_type::Value> = None;

        while true {
            if aKey.is_none() && aData.hasNext() {
                let res = aData.Next().unwrap();
                aKey = Some(res.key);
                aValue = Some(res.value);
            }

            if bKey.is_none() && bData.hasNext() {
                let res = bData.Next().unwrap();
                bKey = Some(res.key);
                bValue = Some(res.value);
            }

            if aKey.is_none() && bKey.is_none() && aData.hasNext() == false && bData.hasNext() == false {
                break;
            }

            let mut k: data_type::Key;
            let mut v: data_type::Value;

            if !aKey.is_none() && !bKey.is_none() {
                let cmp = aKey.clone().unwrap().cmp(&bKey.clone().unwrap());
                
                if cmp == Ordering::Equal {
                    k = bKey.unwrap();
                    v = bValue.unwrap();
                    aKey = None;
                    aValue = None;
                    bKey = None;
                    bValue = None;
                } else if cmp == Ordering::Greater {
                    k = bKey.unwrap();
                    v = bValue.unwrap();

                    bKey = None;
                    bValue = None;
                } else {
                    k = aKey.unwrap();
                    v = aValue.unwrap();

                    aKey = None;
                    aValue = None;
                }
            } else if !aKey.is_none() {
                k = aKey.unwrap();
                v = aValue.unwrap();

                aKey = None;
                aValue = None;
            } else {
                k = bKey.unwrap();
                v = bValue.unwrap();

                bKey = None;
                bValue = None;
            }
            
            block_data.append(&mut k.clone());
            block_data.append(&mut v.clone());
            cnt += 1;

            if cnt == self.entry_num_per_block {
                self.cur_block_id += 1;
                self.bufcache.borrow_mut().write(0, self.cur_block_id, self.trans(block_data.clone()));
                
                cnt = 0;
                block_data.clear();
            }
        }

        block_data.append(&mut self.EOF_KEY.clone());
        block_data.append(&mut self.EOF_VALUE.clone());
        cnt += 1;

        self.cur_block_id += 1;
        self.bufcache.borrow_mut().write(0, self.cur_block_id, self.trans(block_data));

        self.sstable_num -= 1;
    }

}