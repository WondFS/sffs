




pub struct SSTable {
    pub max_key: Key,
    pub mapping: Option<MmapMut>,
    pub mapping_file: Option<File>,
    pub size: u64,
    pub max_size: u64,
    pub level_index: usize,
}