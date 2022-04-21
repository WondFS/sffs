use crate::raw::raw_node::RawNodeDataEntry;

pub struct RawFile {
    pub file_size: u32,
    pub count: u8,
    pub ino: u32,
    pub uid: u16,
    pub gid: u16,
    pub version: u32,
    pub pointers: Option<Vec<RawNodeDataEntry>>,
}

impl RawFile {
    pub fn new() -> RawFile {
        RawFile {
            file_size: 0,
            count: 0,
            ino: 0,
            uid: 0,
            gid: 0,
            version: 0,
            pointers: Some(vec![]),
        }
    }
}