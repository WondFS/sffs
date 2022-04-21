pub enum RawNodeType {
    Indirect,
    Direct,
    Inline,
}

pub struct RawNode {
    pub is_obsolete: bool,
    pub node_type: RawNodeType,
    pub file_size: u32,
    pub count: u8,
    pub ino: u32,
    pub uid: u16,
    pub gid: u16,
    pub version: u32,
    pub address: u32,
    pub inline_data: Option<Vec<u8>>,
    pub indirect_pointers: Option<Vec<RawNodeNodeEntry>>,
    pub pointers: Option<Vec<RawNodeDataEntry>>,
}

pub struct RawNodeNodeEntry {
    pub address: u32,
}

// #[derive(Copy)]
pub struct RawNodeDataEntry {
    pub len: u32,
    pub size: u32,
    pub offset: u32,
    pub address: u32,
}

struct NodeRegion {
    count: usize,
    data: [u8; 3072],
}

impl NodeRegion {
    fn new(data: [u8; 3072],) -> NodeRegion {
        NodeRegion {
            count: 0,
            data
        }
    }
}

impl Iterator for NodeRegion {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 3072 || self.data[self.count+1] == 0 {
            None
        } else {
            self.count += 1;
            Some(0)
        }
    }
}

struct DataRegion {
    count: usize,
    data: [u8; 3072],
}

impl DataRegion {
    fn new(data: [u8; 3072]) -> DataRegion {
        DataRegion {
            count: 0,
            data
        }
    }
}

impl Iterator for DataRegion {
    type Item = (u32, u32, u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 3072 || self.data[self.count+1] == 0 {
            None
        } else {
            self.count += 1;
            Some((0, 0, 0, 0))
        }
    }
}

impl RawNode {
    pub fn new(buf: [u8; 4096]) -> Option<RawNode> {
        let metadata = [0u8; 100];
        let data = [0u8; 3072];
        let mut entries: Vec<RawNodeDataEntry> = vec![];
        let iter = DataRegion::new(data);
        for (len, offset, address, size) in iter {
            let entry = RawNodeDataEntry {
                len,
                offset,
                address,
                size,
            };
            entries.push(entry);
        }
        None
    }

    pub fn decode(&self) -> [u8; 4096] {
        todo!()
    }
}