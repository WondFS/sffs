




pub struct Memtable {
    pub max_size: usize,
    pub entries: BTreeSet<EntryT>,
}

impl memtable {
    pub fn new(size: usize) -> Memtable {
    
    }

    pub fn get(&self, key: &KeyT) -> Option<ValueT> {
    
    }

    pub fn put(&mut self, key: KeyT, value: ValueT) {
        
    }

    pub fn full(&self) -> bool {
        
    }
}