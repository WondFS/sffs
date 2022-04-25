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
        }
    }

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

    pub fn modify_stat(&mut self, stat: InodeStat) -> bool {
        let mut event_group = inode_event::InodeEventGroup::new();
        event_group.inode = self.copy_inode();
        let event = inode_event::ModifyInodeStatInodeEvent {
            file_type: stat.file_type,
            ino: stat.ino,
            size: stat.size,
            uid: stat.uid,
            gid: stat.gid,
            n_link: stat.n_link,
        };
        event_group.events.push(inode_event::InodeEvent::ModifyStat(event));
        let inode = self.core.dispose_event_group(event_group).unwrap();
        self.update_by_another_inode(inode);
        true
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
        event_group.inode = self.copy_inode();
        let mut index = 0;
        let mut flag = false;
        let new_entry = InodeEntry {
            offset,
            len,
            valid: false,
            size: len,
            address: 0,
        };
        let mut second_entry = None;
        let mut second_index = 0;
        if offset > self.size {
            return false;
        }
        for entry in self.data.iter_mut() {
            if entry.offset + entry.len < new_entry.offset {
                index += 1;
                continue
            } else if entry.offset > new_entry.offset + new_entry.len {
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
                        offset: entry.offset,
                        len: valid_prev,
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
                    index += 1;
                    flag = true;
                }
                if valid_suffix > 0 {
                    second_entry = Some(InodeEntry {
                        offset: entry.offset + entry.len - valid_suffix,
                        len: valid_suffix,
                        valid: false,
                        size: valid_suffix / 4096,
                        address: 0,
                    });
                    second_index = index;
                }
            }
        }
        if second_entry.is_some() {
            let second_entry = second_entry.unwrap();
            let data = self.read_entry(&second_entry, second_entry.offset, second_entry.offset + second_entry.offset);
            let event = inode_event::AddContentInodeEvent {
                index: second_index,
                offset: second_entry.offset,
                len: second_entry.len,
                size: second_entry.size,
                content: data,
            };
            event_group.events.push(inode_event::InodeEvent::AddContent(event));
        }
        let inode = self.core.dispose_event_group(event_group).unwrap();
        self.update_by_another_inode(inode);
        true
    }

    pub fn insert(&mut self, offset: u32, len: u32, buf: &Vec<u8>) -> bool {
        let mut event_group = inode_event::InodeEventGroup::new();
        event_group.inode = self.copy_inode();
        let mut index = 0;
        let mut flag = false;
        let new_entry = InodeEntry {
            offset,
            len,
            valid: false,
            size: len,
            address: 0,
        };
        let mut second_entry = None;
        let mut second_index = 0;
        if offset > self.size {
            return false;
        }
        for entry in self.data.iter_mut() {
            if flag {
                let event = inode_event::ChangeContentInodeEvent {
                    index: index,
                    offset: entry.offset + len,
                    v_address: entry.address,
                };
                event_group.events.push(inode_event::InodeEvent::ChangeContent(event));
            } else {
                if new_entry.offset < entry.offset + entry.len {
                    flag = true;
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
                            offset: entry.offset,
                            len: valid_prev,
                            size: valid_prev / 4096,
                            o_size: entry.size,
                            v_address: entry.address,
                        };
                        event_group.events.push(inode_event::InodeEvent::TruncateContent(event));
                    }
                    index  += 1;
                    let event = inode_event::AddContentInodeEvent {
                        index,
                        offset,
                        len,
                        size: len / 4096,
                        content: buf.clone(),
                    };
                    event_group.events.push(inode_event::InodeEvent::AddContent(event));
                    index += 1;
                    if valid_suffix > 0 {
                        second_entry = Some(InodeEntry {
                            offset: entry.offset + entry.len - valid_suffix,
                            len: valid_suffix,
                            valid: false,
                            size: valid_suffix / 4096,
                            address: 0,
                        });
                        second_index = index;
                    }
                }
            }
            index += 1;
        }
        if second_entry.is_some() {
            let second_entry = second_entry.unwrap();
            let data = self.read_entry(&second_entry, second_entry.offset, second_entry.offset + second_entry.offset);
            let event = inode_event::AddContentInodeEvent {
                index: second_index,
                offset: second_entry.offset,
                len: second_entry.len,
                size: second_entry.size,
                content: data,
            };
            event_group.events.push(inode_event::InodeEvent::AddContent(event));
        }
        let inode = self.core.dispose_event_group(event_group).unwrap();
        self.update_by_another_inode(inode);
        true
    }

    pub fn truncate(&mut self, offset: u32, len: u32) -> bool {
        let mut event_group = inode_event::InodeEventGroup::new();
        event_group.inode = self.copy_inode();
        let mut new_entry = None;
        let mut new_index = 0;
        let mut index = 0;
        for entry in self.data.iter_mut() {
            if entry.offset + entry.len < offset {
                index += 1;
                continue
            } else if entry.offset > offset + len {
                let event = inode_event::ChangeContentInodeEvent {
                    index,
                    offset: entry.offset - len,
                    v_address: entry.address,
                };
                event_group.events.push(inode_event::InodeEvent::ChangeContent(event));
                index += 1;
                continue
            } else {
                let valid_prev = max(0, offset - entry.offset);
                let valid_suffix = max(0, entry.offset + entry.len - offset - len);
                if valid_prev == 0 {
                    let event = inode_event::DeleteContentInodeEvent {
                        index,
                        size: entry.size,
                        v_address: entry.address,
                    };
                    event_group.events.push(inode_event::InodeEvent::DeleteContent(event));
                } else {
                    let event = inode_event::TruncateContentInodeEvent {
                        index: (index as u32),
                        offset: entry.offset,
                        len: valid_prev,
                        size: valid_prev / 4096,
                        o_size: entry.size,
                        v_address: entry.address,
                    };
                    event_group.events.push(inode_event::InodeEvent::TruncateContent(event));
                }
                if valid_suffix > 0 {
                    new_entry = Some(InodeEntry {
                        offset: entry.offset + entry.len - valid_suffix,
                        len: valid_suffix,
                        valid: false,
                        size: valid_suffix / 4096,
                        address: 0,
                    });
                    new_index = index;
                    index += 1;
                }
                index += 1;
            }
        }
        if new_entry.is_some() {
            let new_entry = new_entry.unwrap();
            let data = self.read_entry(&new_entry, new_entry.offset, new_entry.offset + new_entry.offset);
            let event = inode_event::AddContentInodeEvent {
                index: new_index,
                offset: new_entry.offset,
                len: new_entry.len,
                size: new_entry.size,
                content: data,
            };
            event_group.events.push(inode_event::InodeEvent::AddContent(event));
        }
        let inode = self.core.dispose_event_group(event_group).unwrap();
        self.update_by_another_inode(inode);
        true
    }

    pub fn truncate_to_end(&mut self, offset: u32) -> bool {
        self.truncate(offset, self.size - offset)
    }

    pub fn dup(&mut self) -> bool {
        let stat = InodeStat {
            file_type: self.file_type,
            ino: self.ino,
            size: self.size,
            uid: self.uid,
            gid: self.gid,
            ref_cnt: self.ref_cnt + 1,
            n_link: self.n_link,
        };
        self.modify_stat(stat)
    }

    pub fn delete(&mut self) -> bool {
        let mut event_group = inode_event::InodeEventGroup::new();
        event_group.inode = self.copy_inode();
        event_group.need_delete = true;
        let inode = self.core.dispose_event_group(event_group).unwrap();
        self.update_by_another_inode(inode);
        true
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
            core: core_manager::CoreManager::new(),
        }
    }
}
