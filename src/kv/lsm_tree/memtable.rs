use std::collections::BTreeSet;
use crate::kv::lsm_tree::data_type;
use crate::kv::lsm_tree::data_type::{Entry, Key, Value};



pub struct Memtable {
    pub threshold: u32,
    pub entries: BTreeSet<Entry>,
}

impl Memtable {
    pub fn new(threshold: u32) -> Memtable {
        Memtable {
            threshold: threshold,
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

    pub fn put(&mut self, key: &Key, value: &Value) {
        let query = Entry {
            key: key.clone(),
            value: value.clone(),
        };
        self.entries.replace(query);
    }

    pub fn flush(&mut self) -> Vec<Entry> {
        let mut entries: Vec<Entry> = Vec::new();

        for entry in &self.entries {
            entries.push(entry.clone());
        }
    
        self.entries.clear();

        entries
    }

    pub fn full(&self) -> bool {
        if self.entries.len() == self.threshold as usize {
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}