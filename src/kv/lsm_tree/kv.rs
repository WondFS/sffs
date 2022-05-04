
pub struct LSMTree {
    levels: Vec<level::Level>,
    memtable: memtable::Memtable,
    worker_pool: threadpool::TreadPool,
    bf_bits_per_entry: f32,
    depth: u64,
    directory: lsm_file::LsmFile,
}

impl LSMTree {
    pub fn new(buf_max_entries: u64, dep: u64, fanout: u64, 
        bf_bits_per_entry: f32, num_threads: u64) -> LSMTree {
            
        }

    pub fn get_run(&mut self, mut run_id: usize) -> Option<&mut run::Run> {

    }

    pub fn num_runs(&self) -> usize {
    
    }

    fn merge_down(&mut self, cur_level: usize) {
    
    }
    
    pub fn put(&mut self, key: &str, value: &str) -> bool {
        
    }

    pub fn get(&mut self, key: &str) -> Option(String) {
        
    }

    pub fn delete(&mut self, key: &str) {
        
    }
}