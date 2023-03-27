use std::io::Cursor;

#[test]
fn lolll() {
    let mut raw_string = 124i32.to_be_bytes();
    println!("network: {:?}", raw_string);

    println!("normal: {:?}", i32::from_be_bytes(raw_string));
}