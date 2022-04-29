
pub fn run() {
    let read: u16 = 0b0100; // 4
    let write: u16 = 0b0010; // 2
    let exec: u16 = 0b0001; // 1

    let owner = 6;
    let group = 3;
    let world = 0;

    let permissions: u16 = 0b0110100100; // 644  rw-r--r--

    println!("=== Bit Ops ===");
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

