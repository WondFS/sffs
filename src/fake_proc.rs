use std::cell::RefCell;
use std::sync::Arc;
use crate::inode::inode_manager;
use crate::common::file_table;
use crate::common::file;
use crate::inode::inode;

// Per-process state
pub struct Proc {
    pub file: Vec<file_table::FileLink>,
    pub cwd: inode_manager::InodeLink,
    pub max_file: u32,
}

pub fn my_proc() -> Proc {
    let mut file = vec![];
    for _ in 0..30 {
        file.push(Arc::new(RefCell::new(file::File::new())));
    }
    Proc {
        file,
        cwd: Arc::new(RefCell::new(inode::Inode::new())),
        max_file: 16,
    }
}