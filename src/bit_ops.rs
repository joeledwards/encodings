
pub fn run() {
    let read: u16 = 0b0100; // 4
    let write: u16 = 0b0010; // 2
    let exec: u16 = 0b0001; // 1

    let owner = 6;
    let group = 3;
    let world = 0;

    let permissions: u16 = 0b0110100100; // 644  rw-r--r--

    println!("=== Bit Ops ===");
    println!("129 = {:08b}", 129u8);
    println!("129 << 1 = {} ({:08b})", 129u8 << 1, 129u8 << 1);
    println!("129.rotate_left(1) = {} ({:08b})", 129u8.rotate_left(1), 129u8.rotate_left(1));
    println!("129 >> 1 = {} ({:08b})", 129u8 >> 1, 129u8 >> 1);
    println!("129.rotate_right(1) = {} ({:08b})", 129u8.rotate_right(1), 129u8.rotate_right(1));
    println!("129 & 1 = {} ({:08b})", 129u8 & 1, 129u8 & 1);
    println!("129 | 1 = {} ({:08b})", 129u8 | 1, 129u8 | 1);
    println!("129 ^ 1 = {} ({:08b})", 129u8 ^ 1, 129u8 ^ 1);
    println!("129 & 2 = {} ({:08b})", 129u8 & 2, 129u8 & 2);
    println!("129 | 2 = {} ({:08b})", 129u8 | 2, 129u8 | 2);
    println!("129 ^ 2 = {} ({:08b})", 129u8 ^ 2, 129u8 ^ 2);
    println!("129 & 129 = {} ({:08b})", 129u8 ^ 129, 129u8 ^ 129);
    println!("129 | 129 = {} ({:08b})", 129u8 ^ 129, 129u8 ^ 129);
    println!("129 ^ 129 = {} ({:08b})", 129u8 ^ 129, 129u8 ^ 129);
    println!("!129 = {} ({:08b})", !129u8, !129u8);
    println!("");
    let a: u8 = 0b00100111;
    let b: u8 = 0b00010111;
    let r: u8 = a ^ b;
    println!("{:08b} ^ {:08b} = {:08b}", a, b, r);
    println!("");
    println!("Permission rw-r--r-- | 644 | 110100100");
    println!("Can owner read? {}", (permissions & (read << owner)) > 0);
    println!("Can owner write? {}", (permissions & (write << owner)) > 0);
    println!("Can owner execute? {}", (permissions & (exec << owner)) > 0);
    println!("Can group read? {}", (permissions & (read << group)) > 0);
    println!("Can group write? {}", (permissions & (write << group)) > 0);
    println!("Can group execute? {}", (permissions & (exec << group)) > 0);
    println!("Can world read? {}", (permissions & (read << world)) > 0);
    println!("Can world write? {}", (permissions & (write << world)) > 0);
    println!("Can world execute? {}", (permissions & (exec << world)) > 0);
    println!("");
}
