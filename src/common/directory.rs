use crate::inode::inode;
use crate::inode::inode_manager;

// Look for a directory entry in a directory
pub fn dir_lookup(i_manager: &mut inode_manager::InodeManager, inode: inode_manager::InodeLink, name: String) -> Option<(inode_manager::InodeLink, usize)> {
    if inode.borrow().file_type != inode::InodeFileType::Directory {
        return None;
    }
    let mut buf = vec![];
    if inode.borrow().read_all(&mut buf) == 0 {
        return None;
    }
    let iter = DirectoryParser::new(&buf);
    for (i, entry) in iter.enumerate() {
        if entry.ino == 0 {
            continue;
        }
        if entry.file_name == name {
            let inode = i_manager.i_get(entry.ino).unwrap();
            return Some((inode, i));
        }
    }
    None
}

// Write a new directory entry (name, ino) into the directory inode.
pub fn dir_link(i_manager: &mut inode_manager::InodeManager, inode: inode_manager::InodeLink, ino: u32, name: String) -> bool {
    if dir_lookup(i_manager, inode.clone(), name.clone()).is_some() {
        return false;
    }
    let mut buf = vec![];
    if inode.borrow().read_all(&mut buf) == 0 {
        return false;
    }
    let iter = DirectoryParser::new(&buf);
    let mut index = 0;
    let per_size = iter.per_size;
    for entry in iter {
        if entry.ino == 0 {
            break;
        }
        index += 1;
    }
    let entry = DirectoryInodeEntry {
        file_name: name,
        ino,
    };
    let buf = DirectoryParser::encode(&entry).unwrap();
    inode.borrow_mut().write((index * per_size) as u32, per_size, &buf)
}

// Delete a directory entry (name, ino) into the directory inode.
pub fn dir_unlink(i_manager: &mut inode_manager::InodeManager, inode: inode_manager::InodeLink, ino: u32, name: String) -> bool {
    if !dir_lookup(i_manager, inode.clone(), name.clone()).is_some() {
        return false;
    }
    let mut buf = vec![];
    if inode.borrow().read_all(&mut buf) == 0 {
        return false;
    }
    let iter = DirectoryParser::new(&buf);
    let mut index = 0;
    let per_size = iter.per_size;
    let len = iter.len;
    for entry in iter {
        if entry.ino == ino && entry.file_name == name {
            break;
        }
        index += 1;
    }
    if index == len {
        return false;
    }
    inode.borrow_mut().truncate((index * per_size) as u32, per_size)
}

pub struct DirectoryInodeEntry {
    pub file_name: String,
    pub ino: u32,
}

pub struct DirectoryParser {
    pub count: usize,
    pub data: Vec<u8>,
    pub len: u32,
    pub per_size: u32,
}

impl DirectoryParser {
    pub fn new(data: &Vec<u8>) -> DirectoryParser {
        DirectoryParser {
            count: 0,
            data: data.clone(),
            len: 0,
            per_size: 14, // 4字节 ino 10字节 name
        }
    }
    
    pub fn decode(buf: &Vec<u8>) -> Option<DirectoryInodeEntry> {
        None
    }
    
    pub fn encode(entry: &DirectoryInodeEntry) -> Option<Vec<u8>> {
        Some(vec![])
    }
}

impl Iterator for DirectoryParser {
    type Item = DirectoryInodeEntry;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}