use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::core::core_manager;
use crate::inode::inode::Inode;
use crate::util::lru_cache;

pub struct InodeManager {
    pub size: usize,
    pub capacity: usize,
    pub core_manager: core_manager::CoreManager,
    pub inode_buffer: Vec<InodeLink>,
    pub lock: Mutex<bool>,
}

pub type InodeLink = Arc<RefCell<Inode>>;

impl InodeManager {
    pub fn new() -> InodeManager {
        let mut buf = vec![];
        for _ in 0..30 {
            buf.push(Arc::new(RefCell::new(Inode::new())));
        }
        let capacity = 30;
        InodeManager {
            size: 0,
            capacity: capacity as usize,
            core_manager: core_manager::CoreManager::new(),
            inode_buffer: buf,
            lock: Mutex::new(false),
        }
    }

    // Allocate an inode on device dev.
    // Mark it as allocated by giving it type type.
    // Returns an unlocked but allocated and referenced inode.
    pub fn i_alloc(&mut self) -> Option<InodeLink> {
        let mut empty_index = -1;
        let _ = self.lock.lock();
        for (index, ip) in self.inode_buffer.iter().enumerate() {
            if empty_index == -1 && ip.borrow().ref_cnt == 0 {
                empty_index = index as i32;
            }
        }
        if empty_index == -1 {
            panic!("InodeManager: alloc no spare cache to store");
        }
        let mut inode = self.core_manager.allocate_inode();
        inode.ref_cnt = 1;
        let link = Arc::new(RefCell::new(inode));
        self.inode_buffer[empty_index as usize] = Arc::clone(&link);
        Some(link)
    }

    // Find the inode with number ino on device dev
    // and return the in-memory copy.
    pub fn i_get(&mut self, ino: u32) -> Option<InodeLink> {
        let mut empty_index = -1;
        let _ = self.lock.lock();
        for (index, ip) in self.inode_buffer.iter().enumerate() {
            if ip.borrow().ref_cnt > 0 && ip.borrow().ino == ino {
                ip.borrow_mut().ref_cnt += 1;
                return Some(Arc::clone(ip));
            }
            if empty_index == -1 && ip.borrow().ref_cnt == 0 {
                empty_index = index as i32;
            }
        }
        if empty_index == -1 {
            panic!("InodeManager: get no spare cache to store");
        }
        let mut inode = self.core_manager.get_inode(ino);
        inode.ref_cnt = 1;
        let link = Arc::new(RefCell::new(inode));
        self.inode_buffer[empty_index as usize] = Arc::clone(&link);
        Some(link)
    }

    // Increment reference count for ip.
    pub fn i_dup(&mut self, inode: &InodeLink) -> InodeLink {
        let _ = self.lock.lock();
        inode.borrow_mut().ref_cnt += 1;
        Arc::clone(inode)
    }

    // Drop a reference to an in-memory inode.
    // If that was the last reference, the inode cache entry can
    // be recycled.
    pub fn i_put(&mut self, inode: InodeLink) {
        let _ = self.lock.lock();
        if inode.borrow().ref_cnt == 0 {
            panic!("InodeManager: put not valid inode");
        }
        inode.borrow_mut().ref_cnt -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut manager = InodeManager::new();
        manager.core_manager.mount();
        let link = manager.i_alloc();
        assert_eq!(link.unwrap().borrow().ino, 1);
        let link = manager.i_alloc();
        assert_eq!(link.unwrap().borrow().ino, 2);
        let link = manager.i_get(2);
        assert_eq!(link.as_ref().unwrap().borrow().ino, 2);
        assert_eq!(link.as_ref().unwrap().borrow().ref_cnt, 2);
        let link = manager.i_dup(link.as_ref().unwrap());
        assert_eq!(link.as_ref().borrow().ref_cnt, 3);
        manager.i_put(link);
        let link = manager.i_get(2);
        assert_eq!(link.as_ref().unwrap().borrow().ref_cnt, 3);
    }
}