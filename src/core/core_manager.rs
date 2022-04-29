use std::sync::Mutex;
use crate::buf;
use crate::core::bit;
use crate::core::pit;
use crate::core::vam;
use crate::inode::inode;
use crate::inode::inode_event;
use crate::kv::fake_kv;
use crate::kv::raw_inode;
use crate::gc::gc_manager;
use crate::gc::gc_event;
use crate::gc::gc_manager::PageUsedStatus;

pub struct CoreManager {
    bit: bit::BIT,
    pit: pit::PIT,
    vam: vam::VAM,
    kv: fake_kv::FakeKV,
    gc: gc_manager::GCManager,
    buf_cache: buf::BufCache,
}

impl CoreManager {
    pub fn new() -> CoreManager {
        CoreManager {
            bit: bit::BIT::new(),
            pit: pit::PIT::new(),
            vam: vam::VAM::new(),
            kv: fake_kv::FakeKV::new(),
            gc: gc_manager::GCManager::new(),
            buf_cache: buf::BufCache::new(),
        }
    }

    pub fn read_sb(&mut self) {
        todo!()
    }

    pub fn mount(&mut self) {
        todo!()
    }
}

// KV Module
impl CoreManager {
    pub fn allocate_inode(&mut self) -> inode::Inode {
        let raw_inode = self.kv.allocate_inode(0);
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

    pub fn get_raw_inode(&mut self, ino: u32) -> raw_inode::RawInode {
        self.kv.get_inode(ino)
    }

    pub fn update_raw_inode(&mut self, raw_inode: raw_inode::RawInode) {
        self.kv.update_inode(raw_inode);
    }

    pub fn update_inode(&mut self, inode: inode::Inode) {
        let mut inode = inode;
        for entry in inode.data.iter_mut() {
            entry.address = self.vam.get_physic_address(entry.address).unwrap()
        }
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
        let mut res;
        loop {
            res = self.gc.find_next_pos_to_write(size);
            if res.is_some() {
                break;
            }
            self.forward_gc();
        }
        res.unwrap()
    }

    pub fn forward_gc(&mut self) {
        let gc_group = self.gc.generate_gc_event();
        self.dispose_gc_group(gc_group);
    }

    pub fn background_gc(&mut self) {
        let flag = false;
        loop {
            if flag {
                let gc_group = self.gc.generate_gc_event();
                self.dispose_gc_group(gc_group);
            }
        }
    }

    pub fn set_main_table_page(&mut self, address: u32, status: PageUsedStatus) {
        self.gc.set_table(address, status);
    }

    pub fn dispose_gc_group(&mut self, gc_group: gc_event::GCEventGroup) {
        let mut gc_group = gc_group;
        CoreManager::sort_gc_event(&mut gc_group);
        for event in gc_group.events {
            match event {
                gc_event::GCEvent::Erase(event) => {
                    self.erase_block(event.block_no);
                    let start_index = event.block_no * 128;
                    let end_index = (event.block_no + 1) * 128;
                    for i in start_index..end_index {
                        self.update_bit(i, false);
                        self.clean_pit(i);
                    }
                }
                gc_event::GCEvent::Move(event) => {
                    let o_address = event.o_address;
                    let d_address = event.d_address;
                    let size = 3;
                    let ino = 1;
                    let mut data = vec![];
                    for i in o_address..o_address + size {
                        self.dirty_pit(i);
                        data.push(self.read_page(i));
                        let v_address = self.vam.get_virtual_address(i);
                        if v_address.is_some() {
                            self.vam.update_map(i, v_address.unwrap());
                        }
                    }
                    for i in d_address..d_address + size {
                        self.update_bit(i, true);
                        self.update_pit(i, ino);
                        self.write_page(i, data[(i - d_address) as usize]);
                    }
                    let mut raw_inode = self.get_raw_inode(ino);
                    for entry in raw_inode.data.iter_mut() {
                        if entry.address == o_address {
                            entry.address = d_address;
                            break;
                        }
                    }
                    self.update_raw_inode(raw_inode);
                }
                _ => ()
            }
        }
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

    pub fn dirty_pit(&mut self, address: u32) {
        self.pit.delete_page(address);
        self.set_main_table_page(address, PageUsedStatus::Dirty);
        self.sync_pit();
    }

    pub fn clean_pit(&mut self, address: u32) {
        self.pit.delete_page(address);
        self.set_main_table_page(address, PageUsedStatus::Clean);
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
    pub fn read_page(&mut self, address: u32) -> [u8; 4096] {
        self.buf_cache.read(0, address)
    }

    pub fn read_block(&mut self, block_no: u32) -> [[u8; 4096]; 128] {
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

    pub fn write_page(&mut self, address: u32, data: [u8; 4096]) {
        self.buf_cache.write(0, address, data)
    }

    pub fn write_block(&mut self, block_no: u32, data: [[u8; 4096]; 128]) {
        let mut address = block_no * 128 - 127;
        for data in data.into_iter() {
            self.write_page(address, data);
            address += 1;
        }
    }

    pub fn erase_block(&mut self, block_no: u32) {
        self.buf_cache.erase(0, block_no)
    }
}

// 对上层提供的读写接口
impl CoreManager {
    pub fn read_data(&mut self, v_address: u32) -> [u8; 4096] {
        let address = self.vam.get_physic_address(v_address).unwrap();
        self.read_page(address)
    }

    pub fn dispose_event_group(&mut self, event_group: inode_event::InodeEventGroup) -> Option<inode::Inode> {
        let mut inode = event_group.inode;
        if event_group.need_delete {
            for entry in inode.data.iter() {
                let address = self.vam.get_physic_address(entry.address).unwrap();
                for i in 0..entry.size {
                    self.dirty_pit(address + i);
                    let v_address = self.vam.get_virtual_address(address + i - 1).unwrap();
                    self.vam.delete_map(address, v_address);
                }
            }
            self.kv.delete_inode(inode.ino);
            None
        } else {
            for entry in inode.data.iter_mut() {
                if entry.valid == false {
                    entry.valid = true;
                }
            }
            for event in event_group.events {
                match event {
                    inode_event::InodeEvent::AddContent(event) => {
                        let mut address = self.find_next_pos_to_write(event.size);
                        let mut v_address = self.vam.get_available_address(event.size);
                        let entry = inode::InodeEntry {
                            offset: event.offset,
                            len: event.len,
                            size: event.size,
                            valid: true,
                            address: v_address,
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
                            self.update_bit(address, true);
                            self.update_pit(address, inode.ino);
                            self.vam.insert_map(address, v_address);
                            address += 1;
                            v_address += 1;
                        }
                        inode.data.insert(event.index as usize, entry);

                    }
                    inode_event::InodeEvent::TruncateContent(event) => {
                        let mut entry = inode.data.get_mut(event.index as usize).unwrap();
                        entry.len = event.len;
                        entry.size = event.size;
                        entry.offset = event.offset;
                        let address = self.vam.get_physic_address(event.v_address).unwrap();
                        for i in event.size..event.o_size {
                            self.dirty_pit(address + i - 1);
                            let v_address = self.vam.get_virtual_address(address + i - 1).unwrap();
                            self.vam.delete_map(address, v_address);
                        }

                    }
                    inode_event::InodeEvent::DeleteContent(event) => {
                        let mut entry = inode.data.get_mut(event.index as usize).unwrap();
                        let address = self.vam.get_physic_address(event.v_address).unwrap();
                        for i in 0..event.size {
                            self.dirty_pit(address + i - 1);
                            let v_address = self.vam.get_virtual_address(address + i - 1).unwrap();
                            self.vam.delete_map(address, v_address);
                        }
                        entry.valid = false;
                    }
                    inode_event::InodeEvent::ModifyStat(event) => {
                        inode.file_type = event.file_type;
                        inode.ino = event.ino;
                        inode.size = event.size;
                        inode.uid = event.uid;
                        inode.gid = event.gid;
                        inode.n_link = event.n_link;
                    }
                    _ => ()
                }
            }
            let mut remove_indexs = vec![];
            for index in 0..inode.data.len() {
                if !inode.data.get(index).unwrap().valid {
                    remove_indexs.push(index);
                }
            }
            for index in remove_indexs.into_iter() {
                inode.data.remove(index);
            }
            let mut raw_inode = CoreManager::transfer_inode_to_raw_inode(&inode);
            for entry in raw_inode.data.iter_mut() {
                entry.address = self.vam.get_virtual_address(entry.address).unwrap();
            }
            self.kv.update_inode(raw_inode);
            Some(inode)
        }
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
            core: CoreManager::new(),
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

    pub fn sort_gc_event(event_group: &mut gc_event::GCEventGroup) {
        let len = event_group.events.len();
        for i in 0..len {
            for j in 0..len - 1 - i {
                let index_1 =  event_group.events[j].get_index();
                let index_2 = event_group.events[j+1].get_index();
                if index_1 > index_2 {
                    let temp = event_group.events[j];
                    event_group.events[j] = event_group.events[j+1];
                    event_group.events[j+1] = temp;
                }
            }
        }
    }
}