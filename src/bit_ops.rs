
pub fn run() {
    let READ: u16 = 0b0100; // 4
    let WRITE: u16 = 0b0010; // 2
    let EXEC: u16 = 0b0001; // 1

    let OWNER = 6;
    let GROUP = 3;
    let WORLD = 0;

    let permissions: u16 = 0b0110100100; // 644  rw-r--r--

    println!("=== Bit Ops ===");
    println!("Can owner read? {}", (permissions & (READ << OWNER)) > 0);
    println!("Can owner write? {}", (permissions & (WRITE << OWNER)) > 0);
    println!("Can owner execute? {}", (permissions & (EXEC << OWNER)) > 0);
    println!("Can group read? {}", (permissions & (READ << GROUP)) > 0);
    println!("Can group write? {}", (permissions & (WRITE << GROUP)) > 0);
    println!("Can group execute? {}", (permissions & (EXEC << GROUP)) > 0);
    println!("Can world read? {}", (permissions & (READ << WORLD)) > 0);
    println!("Can world write? {}", (permissions & (WRITE << WORLD)) > 0);
    println!("Can world execute? {}", (permissions & (EXEC << WORLD)) > 0);
    println!("");
}

