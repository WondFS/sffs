use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::inode::inode::Inode;
use crate::gc::gc_manager::GCManager;

pub struct InodeManager<'a> {
    pub gc_manager: GCManager,
    pub inode_buffer: [InodeLink<'a>; 30],
    pub lock: Mutex<bool>,
}

pub type InodeLink<'a> = Arc<RefCell<Inode<'a>>>;

impl InodeManager<'_> {
    pub fn new() -> InodeManager<'static> {
        InodeManager {
            gc_manager: GCManager::new(),
            inode_buffer: [Arc::new(RefCell::new(Inode::new())); 30],
            lock: Mutex::new(false),
        }
    }

    // Allocate an inode on device dev.
    // Mark it as allocated by giving it type type.
    // Returns an unlocked but allocated and referenced inode.
    pub fn i_alloc() {

    }

    // Find the inode with number ino on device dev
    // and return the in-memory copy.
    pub fn i_get(&mut self, ino: u32) -> Option<InodeLink> {
        let mut empty = Arc::clone(&self.inode_buffer[0]);
        let mut empty_flag = false;
        self.lock.lock();
        for ip in self.inode_buffer.iter() {
            if ip.borrow().borrow().ref_cnt > 0 && ip.borrow().borrow().ino == ino {
                ip.borrow().borrow_mut().ref_cnt += 1;
                return Some(Arc::clone(ip));
            }
            if !empty_flag && ip.borrow().borrow().ref_cnt == 0 {
                empty = Arc::clone(ip);
                empty_flag = true;
            }
        }
        if !empty_flag {
            return None;
        }
        let d_inode = self.gc_manager.get_inode(ino).unwrap();
        empty.borrow().borrow_mut().ino = ino;
        empty.borrow().borrow_mut().ref_cnt = 1;
        empty.borrow().borrow_mut().valid = true;
        empty.borrow().borrow_mut().n_link = d_inode.n_link;
        empty.borrow().borrow_mut().file_type = d_inode.file_type;
        empty.borrow().borrow_mut().size = d_inode.size;
        empty.borrow().borrow_mut().data = d_inode.data;
        empty.borrow().borrow_mut().modify_time = d_inode.modify_time;
        Some(empty)
    }

    // Increment reference count for ip.
    pub fn i_dup(&mut self, inode: InodeLink) -> InodeLink {
        self.lock.lock();
        inode.borrow().borrow_mut().ref_cnt += 1;
        Arc::clone(&inode)
    }

    // Drop a reference to an in-memory inode.
    // If that was the last reference, the inode cache entry can
    // be recycled.
    // If that was the last reference and the inode has no links
    // to it, free the inode (and its content) on disk.
    pub fn i_put(&mut self, inode: InodeLink) -> bool {
        self.lock.lock();
        if inode.borrow().ref_cnt == 1 && inode.borrow().borrow().valid && inode.borrow().borrow().n_link == 0 {
            inode.borrow().borrow_mut().valid = false;
            return true;
        }
        inode.borrow().borrow_mut().ref_cnt -= 1;
        false
    }
}