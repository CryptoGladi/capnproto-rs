use byteorder::{ByteOrder, NetworkEndian, NativeEndian, WriteBytesExt, ReadBytesExt, BigEndian};
use std::io::Cursor;

pub enum Value {
    Bool(bool),
    String(String),
}

pub trait CapnprotoDeserialize {
    fn deserialize(&self) -> Vec<u8>;
}

struct Lol {

}

impl CapnprotoDeserialize for Lol {
    fn deserialize(&self) -> Vec<u8> {
        let mut buf = vec![0; 4];
        byteorder::NetworkEndian::write_i32(&mut buf, 1233333);
        
        buf
    }
}

#[test]
fn lolll() {
    let raw_string = "test".as_bytes();
    byteorder::NetworkEndian::write_i16(buf, n)
}