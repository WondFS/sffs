#[derive(Copy, Clone)]
pub struct BlockInfo {
    pub size: u32,
    pub block_no: u32,
    pub reserved_size: u32,
    pub reserved_offset: u32,
}

pub struct BlockTable {
    pub size: u32,
    pub table: Vec<BlockInfo>
}

impl BlockTable {
    pub fn new(size: u32) -> BlockTable {
        let mut table = vec![];
        for i in 0..size {
            let block = BlockInfo {
                size: 128,
                block_no: i,
                reserved_size: 128,
                reserved_offset: 0,
            };
            table.push(block);
        }
        BlockTable {
            size,
            table,
        }
    }

    pub fn clean_page(&mut self, address: u32) {
        let block_no = address / 128;
        if block_no > self.size - 1 {
            panic!("BlockTable: clean at too big address");
        }
        self.table[block_no as usize].reserved_size = 128;
        self.table[block_no as usize].reserved_offset = 0;
    }

    pub fn use_page(&mut self, address: u32) {
        let block_no = address / 128;
        if block_no > self.size - 1 {
            panic!("BlockTable: use at too big address");
        }
        self.table[block_no as usize].reserved_offset += 1;
        self.table[block_no as usize].reserved_size -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut table = BlockTable::new(32);

        table.use_page(0);
        table.use_page(1);
        table.use_page(2);
        table.use_page(3);

        assert_eq!(table.table[0].block_no, 0);
        assert_eq!(table.table[0].reserved_offset, 4);
        assert_eq!(table.table[0].reserved_size, 124);

        table.clean_page(0);
        table.clean_page(1);
        table.clean_page(2);
        table.clean_page(3);

        assert_eq!(table.table[0].reserved_offset, 0);
        assert_eq!(table.table[0].reserved_size, 128);
    }
}