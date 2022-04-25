use crate::inode::inode_manager;

pub enum FileDescriptorType {
    NONE,
    PIPE,
    INODE,
    DEVICE,
}

pub struct File {
    pub off: u32,
    pub ref_cnt: u8,
    pub read_able: u8,
    pub writeable: u8,
    pub fd_type: FileDescriptorType,
    pub inode: inode_manager::InodeLink,
}

impl File {
    pub fn file_alloc() {
    
    }

    pub fn file_dup() {

    }

    pub fn file_close() {

    }

    pub fn file_stat() {

    }

    pub fn file_read() {

    }

    pub fn file_write() {

    }
}