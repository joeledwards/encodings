
fn hash(data: &[u8]) -> u8 {
    let mut digest: u8 = 0x00;

    for value in data {
        digest ^= value;
    }

    digest
}

pub fn run() {
    println!("=== Simple Hash ===");

    let arr1: [u8; 2] = [0x0f, 0xf0];
    let hash1: u8 = hash(&arr1);
    println!("hash({:?}) => {}", arr1, hash1);

    let arr2: [u8; 2] = [0x08, 0x80];
    let hash2: u8 = hash(&arr2);
    println!("hash({:?}) => {}", arr2, hash2);

    let arr3: [u8; 2] = [0x01, 0x10];
    let hash3: u8 = hash(&arr3);
    println!("hash({:?}) => {}", arr3, hash3);

    let arr4: &[u8] = "buzuli".as_bytes();
    let hash4: u8 = hash(arr4);
    println!("hash({:?}) => {}", arr4, hash4);

    println!("");
}
