use std::cmp::Ordering;
use std::hash::Hash;





pub static KEY_LENGTH: usize = 16;
pub static VALUE_LENGTH: usize = 16;
pub static ENTRY_LENGTH: usize = KEY_LENGTH + VALUE_LENGTH;

pub type Key = Vec<u8>;
pub type Value = Vec<u8>;

#[derive(Eq, Default, Clone, Hash)]
pub struct Entry {
    pub key: Key,
    pub value: Value,
}

impl Entry {
    pub fn new(key: Key, value: Value) -> Entry {
        Entry {
            key: key,
            value: value
        }
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

