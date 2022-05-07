use crate::buf;
use crate::memtable;
use crate::level;







pub struct LSMTree {
    levels: Vec<level::Level>,
    memtable: memtable::Memtable,
    depth: u64,
    buf: buf::BufCache,
    sstable_max_size: usize,
}

pub static DEFAULT_TREE_DEPTH: u64 = 3;

impl LSMTree {
    pub fn new(sstable_max_size: u64, depth: u64, fanout: u64) -> LSMTree {
            
    }

    pub fn get_run(&mut self, mut run_id: usize) -> Option<&mut run::Run> {

    }

    pub fn num_runs(&self) -> usize {
    
    }

    fn merge_down(&mut self, cur_level: usize) {
    
    }
    

    fn fill_str(&self, _str: &str, length: usize) -> Vec<u8> {
        let mut res = vec![' ' as u8; length - _str.len()];
        res.extend(_str.as_bytes().to_vec());
        res
    }




    fn vec_to_str(&self, _vec: &Vec<u8>) -> String {
        let res: String = str::from_utf8(_vec).unwrap().trim().to_owned();
        res
    }


    pub fn put(&mut self, key_str: &str, value_str: &str) -> bool {
        let key = self.fill_str(key_str, data_type::KEY_SIZE);
        let value = self.fill_str(value_str, data_type::VALUE_SIZE);
        if self.memtable.full() == false {
            self.memtable.put(key, value);
            return true;
        }

        self.merge_down(0);

        let size = self.sstable_max_size as u64;
        self.levels[0].sstable.push_front(sstable::SSTable::new(size, 0));

        for entry in self.memtable.entries.iter() {
            self.levels[0].sstable[0].push(&entry);
        }

        self.memtable.put(key, value);

        true
    }

    pub fn get(&mut self, key: &str) -> Option(String) {
        
    }

    pub fn delete(&mut self, key: &str) {
        
    }
}