const virtual_driver: bool = true;

pub fn disk_read(block_no: u32) -> [[u8; 4096]; 128] {
    [[0; 4096]; 128]
}

pub fn disk_write(address: u32, data: [u8; 4096]) {

}

pub fn disk_erase(block_no: u32) {

}