




pub struct LsmMetadata {
    metablock: Vec<usize>,
    bufcache: buf::BufCache,
}

impl LsmMetadata {
    pub fn new() -> LsmMetadata {
        LsmMetadata {
            metablock: Vec<usize>::new(),
            bufcache: BufCache::new(),
        }
    }
}