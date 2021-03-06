
pub fn run() {
    let unsigned_byte: u8 = u8::MAX;
    let signed_byte: i8 = -1;
    let unsigned_short: u16 = u16::MAX;
    let signed_short: i16 = -1;
    let unsigned_int: u32 = u32::MAX;
    let signed_int: i32 = -1;
    let unsigned_long: u64 = u64::MAX;
    let signed_long: i64 = -1;

    println!("=== Int Ranges ===");
    println!("u8 => {:08b} ({})", unsigned_byte, unsigned_byte);
    println!("i8 => {:08b} ({})", signed_byte, signed_byte);
    println!("u16 => {:016b} ({})", unsigned_short, unsigned_short);
    println!("i16 => {:016b} ({})", signed_short, signed_short);
    println!("u32 => {:032b} ({})", unsigned_int, unsigned_int);
    println!("i32 => {:032b} ({})", signed_int, signed_int);
    println!("u64 => {:064b} ({})", unsigned_long, unsigned_long);
    println!("i64 => {:064b} ({})", signed_long, signed_long);
    println!("");

    // Two's compliment (for signed integers)
    //
    // Bits:
    // u4 0000 => 0
    // u4 0001 => 1
    // u4 0010 => 2
    // ...
    // u4 1111 => 15
    //
    // i4 0000 => 0
    // i4 0001 => 1
    // i4 0010 => 2
    // ...
    // i4 0111 => 7
    // i4 1000 => -8
    // i4 1001 => -7
    // ...
    // i4 1111 => -1
    //
}
