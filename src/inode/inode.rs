use std::cmp::{max, min};
use std::sync::Mutex;
use crate::gc::gc_manager;

pub trait InodeTrait {
    fn get_stat(&self) -> Stat;
    fn read_all(&self, buf: &mut Vec<u8>) -> i32;
    fn read(&self, offset: u32, len: u32, buf: &mut Vec<u8>) -> i32;
    fn write(&mut self, offset: u32, len: u32, buf: &Vec<u8>) -> bool;
    fn insert(&mut self, offset: u32, len: u32, buf: &Vec<u8>) -> bool;
    fn truncate(&mut self, offset: u32, len: u32) -> bool;
    fn truncate_to_end(&mut self, offset: u32) -> bool;
    fn dup(&mut self) -> bool;
}

pub struct Stat {
    pub file_type: InodeFileType,
    pub ino: u32,
    pub size: u32,
    pub uid: u32,
    pub gid: u16,
    pub n_link: u8,
    pub create_time: u8,
    pub access_time: u8,
    pub modify_time: u8,
}

#[derive(Clone, Copy)]
pub enum InodeFileType {
    File,
    Directory,
    SoftLink,
    HardLink,
}

#[derive(Copy, Clone)]
pub struct InodeEntry {
    pub valid: bool,
    pub offset: u32,
    pub len: u32,
    pub size: u32,
    pub v_address: u32,
}

pub struct Inode<'a> {
    pub valid: bool,
    pub file_type: InodeFileType,
    pub ino: u32,
    pub size: u32,
    pub uid: u32,
    pub gid: u16,
    pub ref_cnt: u8,
    pub n_link: u8,
    pub create_time: u8,
    pub access_time: u8,
    pub modify_time: u8,
    pub lock: Mutex<bool>,
    pub data: Vec<InodeEntry>,
    pub gc_manager: &'a gc_manager::GCManager,
}

impl InodeTrait for Inode<'_> {
    fn get_stat(&self) -> Stat {
        Stat {
            file_type: self.file_type.clone(),
            ino: self.ino,
            size: self.size,
            uid: self.uid,
            gid: self.gid,
            n_link: self.n_link,
            create_time: self.create_time,
            access_time: self.access_time,
            modify_time: self.modify_time
        }
    }

    fn read_all(&self, buf: &mut Vec<u8>) -> i32 {
        self.read(0, self.size, buf)
    }

    fn read(&self, offset: u32, len: u32, buf: &mut Vec<u8>) -> i32 {
        buf.clear();
        let data = &self.data;
        let mut len = len;
        let mut count = 0;
        if offset > self.size {
            return -1;
        }
        if offset + len > self.size {
            len = self.size - offset;
        }
        for entry in data.iter() {
            if len == 0 {
                break
            }
            let cur_count = min(len, entry.len);
            let raw_data = self.gc_manager.get_data(entry.v_address);
            for byte in raw_data.into_iter() {
                buf.push(byte);
            }
            len -= cur_count;
            count += cur_count as i32;
        }
        count
    }

    fn write(&mut self, offset: u32, len: u32, buf: &Vec<u8>) -> bool {
        let data = &mut self.data;
        let mut index = 0;
        let mut insert_flag = false;
        let new_entry = InodeEntry {
            offset,
            len,
            valid: false,
            size: len,
            v_address: 0
        };
        if offset > self.size {
            return false;
        }
        for entry in data {
            if entry.offset + entry.len < new_entry.offset {
                continue
            } else if entry.offset + entry.len > new_entry.offset + new_entry.len {
                continue
            } else {
                if !insert_flag {
                    data.insert(index + 1, new_entry.clone());
                    index += 1;
                    insert_flag = true;
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
                        v_address: 0
                    };
                    data.insert(index + 1, new_entry);
                }
            }
            index += 1;
        }
        true
    }

    fn insert(&mut self, offset: u32, len: u32, buf: &Vec<u8>) -> bool {
        let data = &mut self.data;
        let mut index = 0;
        let mut insert_flag = false;
        let new_entry = InodeEntry {
            offset,
            len,
            valid: false,
            size: len,
            v_address: 0
        };
        if offset > self.size {
            return false;
        }
        for entry in data {
            if insert_flag {
                entry.offset += len;
            } else {
                if new_entry.offset < entry.offset + entry.len {
                    insert_flag = true;
                    data.insert(index+1, new_entry.clone());
                    index += 1;
                    let new_entry = InodeEntry {
                        offset,
                        len,
                        valid: false,
                        size: len,
                        v_address: 0
                    };
                    data.insert(index+1, new_entry);
                }
            }
            index += 1;
        }
        true
    }

    fn truncate(&mut self, offset: u32, len: u32) -> bool {
        self.truncate_to_end(offset)
    }

    fn truncate_to_end(&mut self, offset: u32) -> bool {
        let data = &mut self.data;
        let mut truncate_flag = false;
        if offset > self.size {
            return false;
        }
        for entry in data {
            if truncate_flag {
                entry.len = 0;
                continue
            }
            if entry.offset + entry.len > offset {
                entry.len = offset - entry.offset;
                truncate_flag = true;
            }
        }
        true
    }

    fn dup(&mut self) -> bool {
        true
    }

}

impl Inode<'_> {
    pub fn new() -> Inode<'static> {
        Inode {
            file_type: InodeFileType::File,
            ino: 0,
            size: 0,
            uid: 0,
            gid: 0,
            n_link: 0,
            create_time: 0,
            access_time: 0,
            modify_time: 0,
            data: vec![],
            gc_manager: &gc_manager::GCManager::new(),
            valid: todo!(),
            ref_cnt: todo!(),
            lock: todo!(),
        }
    }

    // 从GC获取数据区域节点
    pub fn get_data(&self, entry: &InodeEntry) {

    }

    // 写入新的InodeEntry
    pub fn write_data_entry(&self, entry: &InodeEntry) {

    }

    // 写入文件元数据
    pub fn write_stat(&self) {

    }

    // 刷新Inode
    pub fn flash_inode(&mut self) {

    }

    pub fn submit(&self) {

    }
}

pub struct InodeEventGroup {
    /// 文件元数据
    pub file_type: InodeFileType,
    pub ino: u32,
    pub size: u32,
    pub uid: u32,
    pub gid: u32,
    pub n_link: u8,
    pub create_time: u8,
    pub access_time: u8,
    pub modify_time: u8,
    pub need_delete: bool,
    /// 内容修改事件
    pub events: Vec<InodeEvent>,
}

pub enum InodeEvent {
    AddContent(AddContentInodeEvent),
    ChangeContent(ChangeContentInodeEvent),
    DeleteContent(u32),
    None, // 只修改文件元数据
}

pub struct AddContentInodeEvent {
    pub offset: u32,
    pub len: u32,
    pub size: u32,
    pub content: Vec<u8>,
}

pub struct ChangeContentInodeEvent {
    pub o_offset: u32,
    pub o_len: u32,  // 以Byte为单位
    pub o_size: u32, // 以Page为单位
    pub offset: u32,
    pub len: u32,
    pub size: u32,
    pub v_address: u32,
}