use hex::{encode, decode};


/**
 * 0000 => '0'
 * 0001 => '1'
 * 0010 => '2'
 * ...
 * 1110 => 'e'
 * 1111 => 'f'
 */
pub fn hex_encode(data: &[u8]) -> String {
    // MSB | char msb = data[0] & 0xf0 >> 4
    // LSB | char lsb = data[0] & 0x0f
    encode(data)
}

/**
 * '0' => 0000 
 * '1' => 0001 
 * '2' => 0010 
 * ...
 * 'e' => 1110 
 * 'f' => 1111 
 */
pub fn hex_decode(data: String) -> Vec<u8> {
    // restored[0] = data[0] << 4 | data[1]
    // restored[1] = data[2] << 4 | data[3]
    let ascii = data.into_bytes();
    decode(ascii).unwrap()
}

pub fn run () {
    let a = "plain";
    let b = hex_encode(a.as_bytes());
    let b_share = hex_encode(a.as_bytes());
    let c_raw = hex_decode(b_share);
    let c = std::str::from_utf8(c_raw.as_slice()).unwrap();

    println!("=== Hexes ===");
    println!("a = '{}'", a);
    println!("b = hex_encode('{}') => '{}' {:?}", a, b, b.as_bytes());
    println!("c = hex_decode('{}') => '{}' {:?}", b, c, c.as_bytes());
    println!("");
}