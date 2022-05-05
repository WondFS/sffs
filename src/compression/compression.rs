pub trait Compression{
    fn encode(data: &Vec<u8>) -> (Vec<u8>, u32);
    fn decode(data: &Vec<u8>) -> (Vec<u8>, u32);
}