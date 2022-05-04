
use std::collections::VecDeque;



pub struct Level {
    pub sstables: VecDeque<sstable::SSTable>,
    pub sstables_number: usize,
    pub sstable_size: usize,
}

impl Level {
    pub fn new(sstables_number: usize, sstable_size: usize) -> Level {
        Level {
            sstables: VecDeque::new(),
            sstables_number: sstables_number,
            sstable_size: sstable_size,
        }
    }

    pub fn remaining(&self) -> usize {
        self.sstables_number - self.sstables.len()
    }
}