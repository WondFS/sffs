



use std::sync::Arc;
use std::cell::RefCell;
use crate::buf;
use crate::kv::lsm_tree::data_type;
use crate::kv::lsm_tree::block_iterator;
use std::cmp::Ordering;
use crate::kv::lsm_tree::sstable_metadata;


pub struct fileIterator {
    blocks: Vec<u32>,
    block_iter: u32,
    iter: block_iterator::BlockDataIterator,
    buf: sstable_metadata::bufLink,
    max_entries_num: u32,
}

impl fileIterator {
    pub fn new(blocks: Vec<u32>, buf: sstable_metadata::bufLink, max_entries_num: u32) -> fileIterator {
        if blocks.len() == 0 {
            panic!("LSM_TREE: fileIterator blocks' len is 0");
        }

        let first_block_id = blocks.get(0).unwrap();

        fileIterator {
            blocks: blocks.clone(),
            block_iter: 0,
            iter: block_iterator::BlockDataIterator::new(*first_block_id, buf.clone(), max_entries_num),
            buf: buf.clone(),
            max_entries_num: max_entries_num,
        }
    }

    pub fn hasNext(&mut self) -> bool {
        if self.iter.hasNext() == false {
            if self.block_iter as usize == self.blocks.len() - 1 {
                return false;
            } else {
                self.block_iter += 1;
                self.iter = block_iterator::BlockDataIterator::new(*self.blocks.get(self.block_iter as usize).unwrap(), self.buf.clone(), self.max_entries_num);
            }
        }
        true
    }

    pub fn Next(&mut self) -> Option<data_type::Entry> {
        if self.hasNext() == false {
            return None;
        }
        
        Some(self.iter.Next())
    }

    pub fn get(&mut self, key: &data_type::Key) -> Option<data_type::Value> {
        while self.hasNext() {
            let other = self.Next().unwrap();
            if key.cmp(&other.key) == Ordering::Equal {
                return Some(other.value);
            }
        }

        None
    }
}