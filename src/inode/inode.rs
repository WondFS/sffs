use std::sync::Mutex;
use std::cmp::{max, min};

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
        }
    }

    pub fn get_stat(&self) -> InodeStat {
        InodeStat {
            file_type: self.file_type.clone(),
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
            // 读出entry中的数据
            //
            len -= cur_count;
            count += cur_count as i32;
            if len == 0 {
                break;
            }
        }
        count
    }

    pub fn write(&mut self, offset: u32, len: u32, buf: &Vec<u8>) -> bool {
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
                continue
            } else if entry.offset + entry.len > new_entry.offset + new_entry.len {
                continue
            } else {
                if !flag {
                    new_entries.push((index + 1, new_entry.clone()));
                    index += 1;
                    flag = true;
                }
                let valid_prev = max(0, new_entry.offset - entry.offset);
                let valid_suffix = max(0, entry.offset + entry.len - new_entry.offset - new_entry.len);
                entry.len = valid_prev;
                if valid_suffix > 0 {
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

}
