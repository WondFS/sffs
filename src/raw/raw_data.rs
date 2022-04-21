pub struct RawData {
    pub ino: u32,
    pub size: u32,
    pub data: Vec<u8>,
}

impl RawData {
    pub fn new(data: Vec<u8>) -> RawData {
        RawData {
            ino: 0,
            size: (data.len() / 4096 + 1) as u32,
            data,
        }
    }

    pub fn get_page(&self, index: u32) -> Option<[u8; 4096]> {
        if index > self.size {
            return None;
        }
        let mut res = [0; 4096];
        let start_index = (index * 4096) as usize;
        let end_index = ((index + 1) * 4096) as usize;
        for (index, byte) in self.data[start_index..end_index].iter().enumerate() {
            res[index] = byte.clone();
        }
        Some(res)
    }
}