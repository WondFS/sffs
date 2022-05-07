use std::collections::BTreeSet;
use crate::kv::lsm_tree::data_type::{Entry, Key, Value};



pub struct Memtable {
    pub size: usize,
    pub entries: BTreeSet<Entry>,
}

impl Memtable {
    pub fn new(size: usize) -> Memtable {
        Memtable {
            size: size,
            entries: BTreeSet::new(),
        }
    }

    pub fn get(&self, key: &Key) -> Option<Value> {
        let query = Entry {
            key: key.clone(),
            value: Value::default(),
        };
        if let Some(entry) = self.entries.get(&query) {
            Some(entry.value.clone())
        } else {
            None
        }
    }

    pub fn put(&mut self, key: Key, value: Value) {
        let query = Entry {
            key: key.clone(),
            value: value,
        };
        self.entries.replace(query);
    }

    pub fn full(&self) -> bool {
        if self.entries.len() == self.size {
            true
        } else {
            false
        }
    }

    pub fn clear(&self) {
        self.entries.clear();
    }
}