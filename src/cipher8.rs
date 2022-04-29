use std::str;

fn encipher(key: u8, plain_text: &[u8]) -> Vec<u8> {
    let mut cipher_text: Vec<u8> = Vec::new();
    for value in plain_text {
        cipher_text.push(value ^ key);
    }
    cipher_text
}

fn decipher(key: u8, cipher_text: &[u8]) -> Vec<u8> {
    let mut plain_text: Vec<u8> = Vec::new();
    for value in cipher_text {
        plain_text.push(value ^ key);
    }
    plain_text
}

pub fn run() {
    let key: u8 = 0x55;
    let wrong: u8 = 0x57;
    let original = "buzuli".as_bytes();
    let cipher_text = encipher(key, original);
    let plain_text = decipher(key, &cipher_text);
    let wrong_key = decipher(wrong, &cipher_text);

    println!("=== Simple Cipher ===");
    println!("    original : {:?} = '{}'", original, str::from_utf8(&original).unwrap());
    println!(" cipher text : {:?} = '{}'", cipher_text, str::from_utf8(&cipher_text).unwrap());
    println!("  plain text : {:?} = '{}'", plain_text, str::from_utf8(&plain_text).unwrap());
    println!("   wrong key : {:?} = '{}'", wrong_key, str::from_utf8(&wrong_key).unwrap());
    println!("");
}

