// Disk I/O

pub struct DiskDriver {

}

impl DiskDriver {
    pub fn new() -> DiskDriver {
        DiskDriver {

        }
    }

    pub fn disk_read(&self, block_no: u32) -> [[u8; 4096]; 128] {
        [[0; 4096]; 128]
    }
    
    pub fn disk_write(&mut self, address: u32, data: [u8; 4096]) {
    }
    
    pub fn disk_erase(&mut self, block_no: u32) {
    
    }
}