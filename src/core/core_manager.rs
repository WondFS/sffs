use std::sync::Mutex;
use crate::buf;
use crate::gc::gc_manager::PageUsedStatus;
use crate::kv::fake;
use crate::kv::raw_inode;
use crate::gc::gc_manager;
use crate::core::bit;
use crate::core::pit;
use crate::core::vam;
use crate::inode::inode;
use crate::inode::inode_event;

pub struct CoreManager {
    bit: bit::BIT,
    pit: pit::PIT,
    vam: vam::VAM,
    kv: fake::FakeKV,
    gc: gc_manager::GCManager,
    buf_cache: buf::BufCache,
}

impl CoreManager {
    pub fn new() -> CoreManager {
        CoreManager {
            bit: bit::BIT::new(),
            pit: pit::PIT::new(),
            vam: vam::VAM::new(),
            kv: fake::FakeKV::new(),
            gc: gc_manager::GCManager::new(),
            buf_cache: buf::BufCache::new(),
        }
    }

    pub fn read_sb(&mut self) {

    }

    pub fn mount() {

    }
}

// KV Module
impl CoreManager {
    pub fn allocate_inode(&mut self) -> inode::Inode {
        let raw_inode = self.kv.allocate_inode();
        CoreManager::transfer_raw_inode_to_inode(&raw_inode)
    }

    pub fn get_inode(&mut self, ino: u32) -> inode::Inode {
        let mut raw_inode = self.kv.get_inode(ino);
        for entry in raw_inode.data.iter_mut() {
            let address = entry.address;
            entry.address = self.vam.get_available_address(entry.size);
            for i in 0..entry.size {
                self.vam.insert_map(address+i, entry.address+i);
            }
        }
        CoreManager::transfer_raw_inode_to_inode(&raw_inode)
    }

    pub fn update_inode(&mut self, inode: inode::Inode) {
        let raw_inode = CoreManager::transfer_inode_to_raw_inode(&inode);
        self.kv.update_inode(raw_inode);
    }

    pub fn delete_inode(&mut self, ino: u32) {
        self.kv.delete_inode(ino);
    }
}

// GC Module
impl CoreManager {

    pub fn find_next_pos_to_write(&mut self, size: u32) -> u32 {
        todo!()
    }

    pub fn forward_gc(&mut self) {
        todo!()
    }

    pub fn background_gc(&mut self) {
        todo!()
    }

    pub fn set_main_table_page(&mut self, address: u32, status: PageUsedStatus) {
        self.gc.set_table(address, status);
    }

}

// 管理BIT Region
impl CoreManager {
    pub fn read_bit(&mut self) {
        let mut data_1 = self.read_block(1);
        let data_2 = self.read_block(2);
        let mut flag = false;
        for page in data_2.iter() {
            for byte in page.iter() {
                if byte & 0b1111_1111 != 0 {
                    flag = true;
                    break;
                }
            }
        }
        if flag {
            self.erase_block(1);
            self.write_block(1, data_2.clone());
            self.erase_block(2);
            data_1 = data_2;
        }
        self.set_bit(data_1);
    }

    pub fn set_bit(&mut self, data: [[u8; 4096]; 128]) {
        for (i, page) in data.iter().enumerate() {
            for (j, byte) in page.iter().enumerate() {
                let mut byte = byte.clone();
                for k in 0..8 {
                    let index = i * 4096 * 8 + j * 8 + k;
                    if byte & 1 == 1 {
                        self.bit.set_page(index as u32, true);
                        self.set_main_table_page(index as u32, PageUsedStatus::Dirty);
                    } else {
                        self.bit.set_page(index as u32, false);
                        self.set_main_table_page(index as u32, PageUsedStatus::Clean);
                    }
                    byte = byte >> 1;
                }
            }
        }
    }

    pub fn update_bit(&mut self, address: u32, status: bool) {
        self.bit.set_page(address, status);
        match status {
            true => self.set_main_table_page(address, PageUsedStatus::Dirty),
            false => self.set_main_table_page(address, PageUsedStatus::Clean),
        }
        self.sync_bit();
    }

    pub fn sync_bit(&mut self) {
        if self.bit.need_sync() {
            let data = self.bit.encode();
            self.write_block(2, data);
            self.write_block(1, data);
            self.erase_block(2);
            self.bit.sync();
        }
    }
}

// 管理PIT Region
impl CoreManager {
    pub fn read_pit(&mut self) {
        let mut data_1 = vec![];
        let mut data_2 = vec![];
        for i in 0..15 {
            data_1.push(self.read_block(2+i));
        }
        for i in 0..15 {
            data_2.push(self.read_block(18+i));
        }
        for i in 0..15 {
            let mut flag = false;
            for page in data_2.get(i).unwrap().iter() {
                for byte in page.iter() {
                    if byte & 0b1111_1111 != 0 {
                        flag = true;
                        break;
                    }
                }
            }
            if flag {
                self.erase_block((2+i) as u32);
                self.write_block((2+i) as u32, data_2.get(i).unwrap().clone());
                self.erase_block((18+i) as u32);
                *data_1.get_mut(i).unwrap() = data_2.get(i).unwrap().clone();
            }
        }
        self.set_pit(data_1.try_into().unwrap())
    }

    pub fn set_pit(&mut self, data: [[[u8; 4096]; 128]; 32]) {
        let iter = pit::DataRegion::new(data);
        for (index, ino) in iter.enumerate() {
            self.pit.set_page(index as u32, ino);
            self.set_main_table_page(index as u32, PageUsedStatus::Busy(ino));
        }
    }

    pub fn update_pit(&mut self, address: u32, status: u32) {
        self.pit.set_page(address, status);
        self.set_main_table_page(address, PageUsedStatus::Busy(status));
        self.sync_pit();
    }

    pub fn sync_pit(&mut self) {
        if self.pit.need_sync() {
            let data = self.pit.encode();
            for (index, block) in data.iter().enumerate() {
                self.write_block((18+index) as u32, block.clone());
                self.write_block((2+index) as u32, block.clone());
                self.erase_block((18+index) as u32);
            }
            self.pit.sync();
        }
    }
}

// 调用下层的接口，对上不可见
impl CoreManager {
    pub fn read_page(&self, address: u32) -> [u8; 4096] {
        self.buf_cache.read(0, address)
    }

    pub fn read_block(&self, block_no: u32) -> [[u8; 4096]; 128] {
        let max_address = block_no * 128 + 1;
        let mut address = max_address - 128;
        let mut block = vec![];
        while address < max_address {
            let page = self.read_page(address);
            address += 1;
            block.push(page);
        }
        block.try_into().unwrap()
    }

    pub fn write_page(&self, address: u32, data: [u8; 4096]) {
        self.buf_cache.write(0, address, data)
    }

    pub fn write_block(&self, block_no: u32, data: [[u8; 4096]; 128]) {
        let mut address = block_no * 128 - 127;
        for data in data.into_iter() {
            self.write_page(address, data);
            address += 1;
        }
    }

    pub fn erase_block(&self, block_no: u32) {
        self.buf_cache.erase(0, block_no)
    }
}

// 对上层提供的读写接口
impl CoreManager {
    pub fn read_data(&self, v_address: u32) -> [u8; 4096] {
        let address = self.vam.get_physic_address(v_address).unwrap();
        self.read_page(address)
    }

    pub fn dispose_event_group(&mut self, event_group: inode_event::InodeEventGroup) -> inode::Inode {
        let mut inode = event_group.inode;
        let mut raw_inode = CoreManager::transfer_inode_to_raw_inode(&inode);
        if event_group.need_delete {

        } else {
            let mut flag = false;
            for event in event_group.events {
                match event {
                    inode_event::InodeEvent::AddContent(event) => {
                        let mut address = self.find_next_pos_to_write(event.size);
                        let entry = raw_inode::RawEntry {
                            offset: event.offset,
                            len: event.len,
                            size: event.size,
                            address,
                        };
                        for i in 0..event.size {
                            let mut page = vec![];
                            for j in 0..4096 {
                                let byte = event.content.get((i * 4096 + j) as usize);
                                if byte.is_some() {
                                    page.push(byte.unwrap().clone());
                                } else {
                                    page.push(0);
                                }
                            }
                            self.write_page(address, page.try_into().unwrap());
                            address += 1;
                        }
                        raw_inode.data.insert(event.index as usize, entry);
                        flag = true;
                    }
                    inode_event::InodeEvent::TruncateContent(event) => {
                        let mut entry = raw_inode.data.get_mut(event.index as usize).unwrap();
                        entry.len = event.len;
                        entry.size = event.size;
                        entry.offset = event.offset;
                        let address = self.vam.get_physic_address(event.v_address);
                        for i in event.size..event.o_size {

                        }
                        flag = true;
                    }
                    inode_event::InodeEvent::DeleteContent(event) => {
                        flag = true;
                    }
                    inode_event::InodeEvent::ModifyStat(event) => {
                        flag = true;
                    }
                    _ => ()
                }
            }
            if flag {
                // 更新kv中的inode
                // self.kv.update_inode(inode.clone());
            }
        }
        // 回传inode前更新回虚拟地址
        for entry in inode.data.iter_mut() {
            if entry.valid {
                let address = entry.address;
                entry.address = self.vam.get_available_address(entry.size);
                for i in 0..entry.size {
                    self.vam.insert_map(address+i, entry.address+i);
                }
            }
        }
        inode
    }
}

impl CoreManager {
    // 注意这里不进行虚拟地址的转换
    pub fn transfer_raw_inode_to_inode(raw_inode: &raw_inode::RawInode) -> inode::Inode {
        let file_type;
        let mut data = vec![];
        match raw_inode.file_type {
            0 => file_type = inode::InodeFileType::File,
            1 => file_type = inode::InodeFileType::Directory,
            2 => file_type = inode::InodeFileType::SoftLink,
            _ => file_type = inode::InodeFileType::HardLink,
        }
        for entry in raw_inode.data.iter() {
            let entry = inode::InodeEntry {
                len: entry.len,
                size: entry.size,
                offset: entry.offset,
                address: entry.address,
                valid: true,
            };
            data.push(entry);
        }
        inode::Inode {
            valid: true,
            ino: raw_inode.ino,
            size: raw_inode.size,
            uid: raw_inode.uid,
            gid: raw_inode.gid,
            ref_cnt: raw_inode.ref_cnt,
            n_link: raw_inode.n_link,
            lock: Mutex::new(false),
            file_type,
            data,
        }
    }
    
    // 注意这里不进行虚拟地址的转换
    pub fn transfer_inode_to_raw_inode(inode: &inode::Inode) -> raw_inode::RawInode {
        let file_type;
        let mut data = vec![];
        match inode.file_type {
            inode::InodeFileType::File => file_type = 0,
            inode::InodeFileType::Directory => file_type = 1,
            inode::InodeFileType::SoftLink => file_type = 2,
            inode::InodeFileType::HardLink => file_type = 3,
        }
        for entry in inode.data.iter() {
            let entry = raw_inode::RawEntry {
                len: entry.len,
                size: entry.size,
                offset: entry.offset,
                address: entry.address,
            };
            data.push(entry);
        }
        raw_inode::RawInode {
            ino: inode.ino,
            uid: inode.uid,
            gid: inode.gid,
            size: inode.size,
            n_link: inode.n_link,
            ref_cnt: inode.ref_cnt,
            file_type,
            data,
        }
    }
}