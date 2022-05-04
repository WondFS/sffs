





pub type Key = Vec<u8>;
pub type Value = Vec<u8>;

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
    fn partial_cmp(other: &Self) -> Option {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(other: &Self) -> bool {
        self.key == other.key
    }
}

