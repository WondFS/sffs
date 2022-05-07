use crate::buf;
use crate::kv::lsm_tree::sstable_metadata;
use crate::kv::lsm_tree::data_type::{Entry, Key, Value};





pub struct SSTable<'a> {
    pub max_key: Key,
    pub metadata: 'a sstable_metadata::SSTableMetadata,
    pub size: u64,
    pub max_size: u64,
    pub level_index: usize,
}



impl SSTable {
    pub fn new(level: usize, bufcache: &BufCache::bufCache) -> SSTable {
        SSTable {
            
        }
    }

    pub fn get(&mut self, key: &Key) -> Option<Value> {
        
    }

    pub fn get_all_key(&mut self) -> Vec<Key> {
        
    }

    pub fn put(&mut self, entry: &Entry) {
        
    }


    
}