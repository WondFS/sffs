use std::collections::HashMap;

pub struct VAM {
    count: u32,
    physical_address_table: HashMap<u32, u32>, // physical -> virtual
    virtual_address_table: HashMap<u32, u32>,  // virtual -> physical
}

impl VAM {
    pub fn new() -> VAM {
        VAM {
            count: 0,
            physical_address_table: HashMap::default(),
            virtual_address_table: HashMap::default(),
        }
    }

    pub fn find_next_available_virtual_address(&mut self) -> u32 {
        let res = self.count;
        self.count += 1;
        res
    }

    pub fn get_virtual_address(&self, address: u32) -> Option<u32> {
        let address = self.physical_address_table.get(&address);
        match address {
            Some(address) => {
                Some(address.clone())
            }
            None => None
        }
    }

    pub fn get_physic_address(&self, v_address: u32) -> Option<u32> {
        let address = self.virtual_address_table.get(&v_address);
        match address {
            Some(address) => {
                Some(address.clone())
            }
            None => None
        }
    }

    pub fn insert_map(&mut self, address: u32, v_address: u32) {
        self.physical_address_table.insert(address, v_address);
        self.virtual_address_table.insert(v_address, address);
    }

    pub fn update_map(&mut self, address: u32, v_address: u32) {
        *self.physical_address_table.get_mut(&address).unwrap() = v_address;
        *self.virtual_address_table.get_mut(&v_address).unwrap() = address;
    }

    pub fn delete_map(&mut self, address: u32, v_address: u32) {
        self.physical_address_table.remove(&address);
        self.virtual_address_table.remove(&v_address);
    }
}