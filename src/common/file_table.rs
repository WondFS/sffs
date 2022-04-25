use std::sync::Mutex;
use crate::common::file;

pub struct FileTable {
    pub lock: Mutex<bool>,
    pub file: Vec<file::File>,
    pub max_num: u32,
}

impl FileTable {
    pub fn new() -> FileTable {
        FileTable {
            lock: Mutex::new(false),
            file: vec![],
            max_num: 100,
        }
    }

}