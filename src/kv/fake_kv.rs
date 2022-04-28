use std::collections::HashMap;
use crate::kv::raw_inode;

pub struct FakeKV {
    pub map: HashMap<u32, raw_inode::RawInode>,
}

impl FakeKV {
    pub fn new() -> FakeKV {
        FakeKV {
            map: HashMap::new(),
        }
    }

    pub fn get_inode(&mut self, ino: u32) -> raw_inode::RawInode {
        let raw_inode = self.map.get(&ino).unwrap();
        let mut data = vec![];
        for entry in raw_inode.data.iter() {
            data.push(*entry);
        }
        raw_inode::RawInode {
            ino,
            data,
            uid: raw_inode.uid,
            gid: raw_inode.gid,
            size: raw_inode.size,
            n_link: raw_inode.n_link,
            ref_cnt: raw_inode.ref_cnt,
            file_type: raw_inode.file_type,
        }
    }

    pub fn update_inode(&mut self, inode: raw_inode::RawInode) {
        let ino = inode.ino;
        *self.map.get_mut(&ino).unwrap() = inode;
    }

    pub fn delete_inode(&mut self, ino: u32) {
        self.map.remove(&ino);
    }

    pub fn allocate_inode(&mut self) -> raw_inode::RawInode {
        let raw_inode = raw_inode::RawInode {
            ino: 0,
            uid: 0,
            gid: 0,
            size: 0,
            n_link: 0,
            ref_cnt: 0,
            file_type: 0,
            data: vec![],
        };
        self.map.insert(0, raw_inode);
        let raw_inode = raw_inode::RawInode {
            ino: 0,
            uid: 0,
            gid: 0,
            size: 0,
            n_link: 0,
            ref_cnt: 0,
            file_type: 0,
            data: vec![],
        };
        raw_inode
    }
}