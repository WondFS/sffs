
pub trait KV<K: Key> {
    fn get<'a, BK: Borrow<K>>(&self, options: ReadOptions<'a, K>, key: BK) -> Result<Option<Vec<u8>>, Error>;

    fn get_bytes<'a, BK: Borrow<K>>(&self, options: ReadOptions<'a, K>, key: BK) -> Result<Option<Bytes>, Error>;

    fn put<BK: Borrow<K>>(&self, options: WriteOptions, key: BK, value: &[u8]) -> Result<(), Error>;

    fn delete<BK: Borrow<K>>(&self, options: WriteOptions, key: BK) -> Result<(), Error>;
}

impl<K: Key> KV<K> for Database<K> {
    fn put<BK: Borrow<K>>(&self, options: WriteOptions, key: BK, value: &[u8]) -> Result<(), Error> {}

    fn delete<BK: Borrow<K>>(&self, options: WriteOptions, key: BK) -> Result<(), Error> {}

    fn get_bytes<'a, BK: Borrow<K>>(&self, options: ReadOptions<'a, K>, key: BK) -> Result<Option<Bytes>, Error> {}

    fn get<'a, BK: Borrow<K>>(&self, options: ReadOptions<'a, K>, key: BK) -> Result<Option<Vec<u8>>, Error> {
        self.get_bytes(options, key).map(|val| val.map(Into::into))
    }
}