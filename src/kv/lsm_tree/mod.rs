pub mod kv;
pub mod sstable_metadata;
pub mod memtable;
pub mod block_iterator;
pub mod file_iterator;
pub mod data_type;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut kv = kv::LSMTree::new();
        kv.put("a", "b");
        kv.put("ssa", "ada");
        assert_eq!(kv.get("a").unwrap(), "b");
        assert_eq!(kv.get("ssa").unwrap(), "ada");
        kv.put("a", "bc");
        assert_eq!(kv.get("a").unwrap(), "bc");
        for i in 0..100 {
            kv.put(&i.to_string(), "test");
        }
        for i in 0..100 {
            assert_eq!(kv.get(&i.to_string()).unwrap(), "test");
        }
    }
}