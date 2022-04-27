// Disk I/O Simulator

pub struct FakeDisk {
    pub size: u32,
    pub data: Vec<[u8; 4096]>,
}

impl FakeDisk {
    pub fn new(size: u32) -> FakeDisk {
        let mut data = vec![];
        if size % 128 != 0 {
            panic!()
        }
        for _ in 0..size {
            data.push([0; 4096]);
        }
        FakeDisk {
            size,
            data,
        }
    }

    pub fn fake_disk_read(block_no: u32) -> [[u8; 4096]; 128] {
        [[0; 4096]; 128]
    }
    
    pub fn fake_disk_write(address: u32, data: [u8; 4096]) {
    
    }
    
    pub fn fake_disk_erase(block_no: u32) {
    
    }
}