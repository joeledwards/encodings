use base32::{Alphabet, encode, decode};

fn standard_no_pad () {
    let a = b"hello world";
    let b = encode(Alphabet::RFC4648 { padding: false }, a);
    let c = decode(Alphabet::RFC4648 { padding: false }, &b).unwrap();

    let a_str = std::str::from_utf8(a).unwrap();
    let b_str = b;
    let c_str = std::str::from_utf8(&c).unwrap();

    println!("base32::encode(Alphabet::RFC4648_NOPAD, {:?}) => \"{}\" {:?}", &a_str, &b_str, b_str.as_bytes());
    println!("base32::decode(Alphabet::RFC4648_NOPAD, {:?}) => \"{}\" {:?}", &b_str, &c_str, c_str.as_bytes());
}

fn standard_pad () {
    let a = b"hello world";
    let b = encode(Alphabet::RFC4648 { padding: true }, a);
    let c = decode(Alphabet::RFC4648 { padding: true }, &b).unwrap();

    let a_str = std::str::from_utf8(a).unwrap();
    let b_str = b;
    let c_str = std::str::from_utf8(&c).unwrap();

    println!("base32::encode(Alphabet::RFC4648_PAD, {:?}) => \"{}\" {:?}", &a_str, &b_str, b_str.as_bytes());
    println!("base32::decode(Alphabet::RFC4648_PAD, {:?}) => \"{}\" {:?}", &b_str, &c_str, c_str.as_bytes());
}

fn crockford () {
    let a = b"hello world";
    let b = encode(Alphabet::Crockford, a);
    let c = decode(Alphabet::Crockford, &b).unwrap();

    let a_str = std::str::from_utf8(a).unwrap();
    let b_str = b;
    let c_str = std::str::from_utf8(&c).unwrap();

    println!("base32::encode(Alphabet::Crockford, {:?}) => \"{}\" {:?}", &a_str, &b_str, b_str.as_bytes());
    println!("base32::decode(Alphabet::Crockford, {:?}) => \"{}\" {:?}", &b_str, &c_str, c_str.as_bytes());
}

pub fn run () {
    println!("=== Base 32 ===");
    standard_no_pad();
    standard_pad();
    crockford();
    println!("");
}
