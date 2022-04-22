use crate::kv::raw_inode;

pub struct FakeKV {

}

impl FakeKV {
    pub fn new() -> FakeKV {
        FakeKV {

        }
    }

    pub fn get_inode(&mut self, ino: u32) -> raw_inode::RawInode {
        todo!()
    }

    pub fn update_inode(&mut self, inode: raw_inode::RawInode) {
        todo!()
    }

    pub fn delete_inode(&mut self, ino: u32) {
        todo!()
    }

    pub fn allocate_inode(&mut self) -> raw_inode::RawInode {
        todo!()
    }
}