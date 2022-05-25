use base64::{encode, decode};

pub fn run () {
    let a = b"hello world";
    let b = encode(a);
    let c = decode(&b).unwrap();

    let a_str = std::str::from_utf8(a).unwrap();
    let b_str = b;
    let c_str = std::str::from_utf8(&c).unwrap();

    println!("=== Base 64 ===");
    println!("base64::encode({:?}) => \"{}\" {:?}", &a_str, &b_str, b_str.as_bytes());
    println!("base64::decode({:?}) => \"{}\" {:?}", &b_str, &c_str, c_str.as_bytes());
    println!("");
}