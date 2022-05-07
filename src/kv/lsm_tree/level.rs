
use std::collections::VecDeque;
use crate::kv::lsm_tree::sstable;



pub struct Level {
    pub sstables: VecDeque<sstable::SSTable>,
    pub sst_num: usize,
    pub sst_max_num: usize,
}

impl Level {
    pub fn new(sst_num: usize, sst_max_num: usize) -> Level {
        Level {
            sstables: VecDeque::new(),
            sst_num: sst_num,
            sst_max_num: sst_max_num,
        }
    }

    pub fn remaining(&self) -> usize {
        self.sst_max_num - self.sstables.len()
    }
}