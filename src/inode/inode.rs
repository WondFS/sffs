use std::sync::Mutex;
use std::cmp::{max, min};
use crate::core::core_manager;
use crate::inode::inode_event;

#[derive(Copy, Clone, PartialEq)]
pub enum InodeFileType {
    File,
    Directory,
    SoftLink,
    HardLink,
}

pub struct InodeStat {
    pub file_type: InodeFileType,
    pub ino: u32,
    pub size: u32,
    pub uid: u32,
    pub gid: u16,
    pub n_link: u8,
}

#[derive(Copy, Clone)]
pub struct InodeEntry {
    pub valid: bool,
    pub offset: u32,
    pub len: u32,           // 以Byte为单位
    pub size: u32,          // 以Page为单位
    pub address: u32,
}

pub struct Inode {
    pub valid: bool,
    pub file_type: InodeFileType,
    pub ino: u32,
    pub size: u32,
    pub uid: u32,
    pub gid: u16,
    pub ref_cnt: u8,
    pub n_link: u8,
    pub lock: Mutex<bool>,
    pub data: Vec<InodeEntry>,
    pub core: core_manager::CoreManager,
    pub event_group: Option<Box<inode_event::InodeEventGroup>>,
}

impl Inode {
    pub fn new() -> Inode {
        Inode {
            file_type: InodeFileType::File,
            ino: 0,
            size: 0,
            uid: 0,
            gid: 0,
            n_link: 0,
            data: vec![],
            valid: false,
            ref_cnt: 0,
            lock: Mutex::new(false),
            core: core_manager::CoreManager::new(),
            event_group: None,
        }
    }

    pub fn get_stat(&self) -> InodeStat {
        InodeStat {
            file_type: self.file_type,
            ino: self.ino,
            size: self.size,
            uid: self.uid,
            gid: self.gid,
            n_link: self.n_link,
        }
    }

    pub fn read_all(&self, buf: &mut Vec<u8>) -> i32 {
        self.read(0, self.size, buf)
    }

    pub fn read(&self, offset: u32, len: u32, buf: &mut Vec<u8>) -> i32 {
        buf.clear();
        let mut len = len;
        let mut count = 0;
        let mut flag = false;
        if offset > self.size {
            return -1;
        }
        if offset + len > self.size {
            len = self.size - offset;
        }
        for entry in self.data.iter() {
            if entry.offset + entry.len < offset {
                continue;
            }
            let cur_count;
            if !flag {
                flag = true;
                cur_count = min(len, entry.offset + entry.len - offset);
            } else {
                cur_count = min(len, entry.len);
            }
            let data = self.read_entry(&entry, 0, cur_count);
            for byte in data.into_iter() {
                buf.push(byte);
            }
            len -= cur_count;
            count += cur_count as i32;
            if len == 0 {
                break;
            }
        }
        count
    }

    pub fn write(&mut self, offset: u32, len: u32, buf: &Vec<u8>) -> bool {
        let mut event_group = inode_event::InodeEventGroup::new();
        let mut index = 0;
        let mut flag = false;
        let mut new_entries = vec![];
        let new_entry = InodeEntry {
            offset,
            len,
            valid: false,
            size: len,
            address: 0,
        };
        if offset > self.size {
            return false;
        }
        for entry in self.data.iter_mut() {
            if entry.offset + entry.len < new_entry.offset {
                index += 1;
                continue
            } else if entry.offset + entry.len > new_entry.offset + new_entry.len {
                continue
            } else {
                let valid_prev = max(0, new_entry.offset - entry.offset);
                let valid_suffix = max(0, entry.offset + entry.len - new_entry.offset - new_entry.len);
                if valid_prev == 0 {
                    let event = inode_event::DeleteContentInodeEvent {
                        index,
                        size: entry.size,
                        v_address: entry.address,
                    };
                    event_group.events.push(inode_event::InodeEvent::DeleteContent(event));
                } else {
                    let event = inode_event::TruncateContentInodeEvent {
                        index,
                        offset,
                        len,
                        size: valid_prev / 4096,
                        o_size: entry.size,
                        v_address: entry.address,
                    };
                    event_group.events.push(inode_event::InodeEvent::TruncateContent(event));
                }
                index += 1;
                if !flag {
                    let event = inode_event::AddContentInodeEvent {
                        index,
                        offset,
                        len,
                        size: len / 4096,
                        content: buf.clone(),
                    };
                    event_group.events.push(inode_event::InodeEvent::AddContent(event));
                    new_entries.push((index as usize, new_entry.clone()));
                    index += 1;
                    flag = true;
                }
                if valid_suffix > 0 {
                    let new_entry = InodeEntry {
                        offset: entry.offset + entry.len - valid_suffix,
                        len: valid_suffix,
                        valid: false,
                        size: valid_suffix / 4096,
                        address: 0,
                    };
                    let offset = entry.offset + entry.len - valid_suffix;
                    let data = vec![];
                    let event = inode_event::AddContentInodeEvent {
                        index,
                        offset,
                        len: valid_suffix,
                        size: valid_suffix / 4096,
                        content: data,
                    };
                    event_group.events.push(inode_event::InodeEvent::AddContent(event));
                    new_entries.push((index as usize, new_entry.clone()));
                    index += 1;
                }
            }
        }
        for entry in new_entries.into_iter() {
            self.data.insert(entry.0, entry.1);
        }
        let inode = self.core.dispose_event_group(event_group).unwrap();
        self.update_by_another_inode(inode);
        true
    }

    pub fn insert(&mut self, offset: u32, len: u32, buf: &Vec<u8>) -> bool {
        let mut index = 0;
        let mut flag = false;
        let mut new_entries = vec![];
        let new_entry = InodeEntry {
            offset,
            len,
            valid: false,
            size: len,
            address: 0,
        };
        if offset > self.size {
            return false;
        }
        for entry in self.data.iter_mut() {
            if flag {
                entry.offset += len;
            } else {
                if new_entry.offset < entry.offset + entry.len {
                    flag = true;
                    new_entries.push((index + 1, new_entry.clone()));
                    index += 1;
                    let new_entry = InodeEntry {
                        offset,
                        len,
                        valid: false,
                        size: len,
                        address: 0,
                    };
                    new_entries.push((index + 1, new_entry));
                }
            }
            index += 1;
        }
        for entry in new_entries.iter() {
            self.data.insert(entry.0, entry.1.clone());
        }
        true
    }

    pub fn truncate(&mut self, offset: u32, len: u32) -> bool {
        self.truncate_to_end(offset)
    }

    pub fn truncate_to_end(&mut self, offset: u32) -> bool {
        let mut flag = false;
        if offset > self.size {
            return false;
        }
        for entry in self.data.iter_mut() {
            if flag {
                entry.len = 0;
                continue
            }
            if entry.offset + entry.len > offset {
                entry.len = offset - entry.offset;
                flag = true;
            }
        }
        true
    }

    pub fn dup(&mut self) {
        
    }

    pub fn delete(&mut self) {

    }
}

impl Inode {
    pub fn read_entry(&self, entry: &InodeEntry, start: u32, end: u32) -> Vec<u8> {
        let start_index = start / 4096;
        let end_index = end / 4096;
        let mut pages = vec![];
        for i in start_index..end_index {
            pages.push(self.core.read_data(entry.address + i));
        }
        let mut res = vec![];
        res.copy_from_slice(&pages[0][start as usize..4096]);
        if end_index - start_index > 1 {
            for i in 1..end_index-1 {
                res.append(&mut pages[i as usize].to_vec());
            }
        }
        let remain_count = (end % 4096) as usize;
        res.append(&mut pages[end_index as usize][0..remain_count].to_vec());
        res
    }

    pub fn update_by_another_inode(&mut self, inode: Inode) {
        self.valid = inode.valid;
        self.file_type = inode.file_type;
        self.ino = inode.ino;
        self.size = inode.size;
        self.uid = inode.uid;
        self.gid = inode.gid;
        self.ref_cnt = inode.ref_cnt;
        self.n_link = inode.n_link;
        self.data = inode.data;
    }
}
