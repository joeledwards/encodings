use hex::{encode, decode};

fn hex_encode(data: &[u8]) -> String {
    encode(data)
}

fn hex_decode(data: &[u8]) -> Vec<u8> {
    decode(data).unwrap()
}

pub fn run () {
    let a = "plain";
    let b = hex_encode(a.as_bytes());
    let c_raw = hex_decode(b.as_bytes());
    let c = std::str::from_utf8(c_raw.as_slice()).unwrap();

    println!("=== Hexes ===");
    println!("a = '{}'", a);
    println!("b = '{}' = hex_encode('{}')", b, a);
    println!("c = '{}' = hex_decode('{}')", c, b);
    println!("");
}