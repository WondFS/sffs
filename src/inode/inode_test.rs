use std::sync::Mutex;
use std::cmp::{max, min};
use crate::inode::inode_event;
use crate::inode::inode_manager;
use crate::compress::compress;

#[derive(Copy, Clone, PartialEq, Debug)]
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
    pub ref_cnt: u8,
    pub n_link: u8,
}

#[derive(Copy, Clone)]
pub struct InodeEntry {
    pub valid: bool,
    pub offset: u32,
    pub len: u32,           // 以Byte为单位
    pub size: u32,          // 以Page为单位
    pub address: u32,
    pub compress_len: u32,  // 压缩后的长度
    pub compress_type: compress::CompressType,
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
    pub core: Option<inode_manager::CoreLink>,
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
            core: None,
        }
    }

    pub fn read_all(&mut self, buf: &mut Vec<u8>) -> i32 {
        self.read(0, self.size, buf)
    }

    pub fn read(&mut self, offset: u32, len: u32, buf: &mut Vec<u8>) -> i32 {
        buf.clear();
        let mut len = len;
        let mut count = 0;
        let mut flag = false;
        if offset >= self.size {
            return -1;
        }
        if offset + len > self.size {
            len = self.size - offset;
        }
        for entry in self.data.clone().iter() {
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
            let data = self.read_decompress_entry(&entry, 0, cur_count);
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
        // event_group.inode = self.copy_inode();
        let mut index = 0;
        let mut flag = false;
        let new_entry = InodeEntry {
            offset,
            len,
            valid: false,
            size: 0,
            address: 0,
            compress_len: 0,
            compress_type: compress::CompressType::None,
        };
        let mut second_entry = None;
        let mut second_o_entry = None;
        let mut second_index = 0;
        if offset > self.size {
            return false;
        }
        for entry in self.data.iter() {
            if entry.offset + entry.len <= new_entry.offset {
                index += 1;
                continue
            } else if entry.offset >= new_entry.offset + new_entry.len {
                continue
            } else {
                let valid_prev = max(0, new_entry.offset as i32 - entry.offset as i32) as u32;
                let valid_suffix = max(0, entry.offset as i32 + entry.len as i32 - new_entry.offset as i32 - new_entry.len as i32) as u32;
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
                        offset: entry.offset,
                        len: valid_prev,
                        size: valid_prev / 4096 + 1,
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
                        size: len / 4096 + 1,
                        content: buf.clone(),
                    };
                    event_group.events.push(inode_event::InodeEvent::AddContent(event));
                    index += 1;
                    flag = true;
                }
                if valid_suffix > 0 {
                    second_o_entry= Some(entry.clone());
                    second_entry = Some(InodeEntry {
                        offset: entry.offset + entry.len - valid_suffix,
                        len: valid_suffix,
                        valid: false,
                        size: valid_suffix / 4096 + 1,
                        address: 0,
                        compress_len: 0,
                        compress_type: compress::CompressType::None,
                    });
                    second_index = index;
                }
            }
        }
        if !flag {
            let event = inode_event::AddContentInodeEvent {
                index: self.data.len() as u32,
                offset: new_entry.offset,
                len: new_entry.len,
                size: len / 4096 + 1,
                content: buf.clone(),
            };
            event_group.events.push(inode_event::InodeEvent::AddContent(event));
        }
        if second_entry.is_some() {
            let second_entry = second_entry.unwrap();
            let data = self.read_entry(&second_o_entry.unwrap(), second_entry.offset - second_o_entry.unwrap().offset, second_entry.offset + second_entry.len - second_o_entry.unwrap().offset);
            let event = inode_event::AddContentInodeEvent {
                index: second_index,
                offset: second_entry.offset,
                len: second_entry.len,
                size: second_entry.size,
                content: data,
            };
            event_group.events.push(inode_event::InodeEvent::AddContent(event));
        }
        let inode = self.core.as_mut().unwrap().borrow_mut().dispose_event_group(event_group).unwrap();
        // self.update_by_another_inode(inode);
        true
    }
}

impl Inode {
    pub fn get_stat(&self) -> InodeStat {
        InodeStat {
            file_type: self.file_type,
            ino: self.ino,
            size: self.size,
            uid: self.uid,
            gid: self.gid,
            ref_cnt: self.ref_cnt,
            n_link: self.n_link,
        }
    }

}

impl Inode {
    pub fn read_decompress_entry(&mut self, entry: &InodeEntry, start: u32, end: u32) -> Vec<u8> {
        if entry.compress_type == compress::CompressType::None {
            self.read_entry(entry, start, end)
        } else {
            let data = self.read_entry(entry, 0, entry.compress_len);
            let data = compress::CompressManager::decode(&data, entry.compress_type);
            data[start as usize..end as usize].to_vec()
        }
    }

    pub fn read_entry(&mut self, entry: &InodeEntry, start: u32, end: u32) -> Vec<u8> {
        let start_index = start / 4096;
        let start_off = start % 4096;
        let end_index = (end - 1) / 4096;
        let end_off = (end - 1) % 4096;
        let mut pages = vec![];
        for i in start_index..end_index + 1 {
            pages.push(self.core.as_mut().unwrap().borrow_mut().read_data(entry.address + i));
        }
        let mut res = vec![];
        if end_index - start_index > 0 {
            for byte in pages[0][start_off as usize..4096].iter() {
                res.push(*byte);
            }
        } else {
            for byte in pages[0][start_off as usize..(end_off + 1) as usize].iter() {
                res.push(*byte);
            }
        }
        if end_index - start_index > 1 {
            for i in 1..end_index - start_index {
                res.append(&mut pages[i as usize].to_vec());
            }
        }
        if end_index - start_index > 0 {
            res.append(&mut pages[(end_index - start_index) as usize][0..(end_off + 1) as usize].to_vec());
        }
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

    pub fn copy_inode(&self) -> Inode {
        Inode {
            file_type: self.file_type,
            ino: self.ino,
            size: self.size,
            uid: self.uid,
            gid: self.gid,
            n_link: self.n_link,
            data: self.data.clone(),
            valid: self.valid,
            ref_cnt: self.ref_cnt,
            lock: Mutex::new(false),
            core: None,
        }
    }

}