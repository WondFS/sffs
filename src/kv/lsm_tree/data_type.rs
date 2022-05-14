use std::cmp::Ordering;
use std::str;
use std::hash::Hash;


pub static KEY_LENGTH: u32 = 16;
pub static VALUE_LENGTH: u32 = 16;
pub static ENTRY_LENGTH: u32 = KEY_LENGTH + VALUE_LENGTH;
pub static TOMBSTONE: &str = "TOMBSTONE";
pub static EOF: &str = "EOF";

pub type Key = Vec<u8>;
pub type Value = Vec<u8>;

#[derive(Eq, Default, Clone, Hash)]
pub struct Entry {
    pub key: Key,
    pub value: Value,
}

pub fn fill_str(_str: &str, length: usize) -> Vec<u8> {
    let mut res = vec![' ' as u8; length - _str.len()];
    res.extend(_str.as_bytes().to_vec());
    res
}

pub fn vec_to_str(_vec: &Vec<u8>) -> String {
    let res: String = str::from_utf8(_vec).unwrap().trim().to_owned();
    res
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

