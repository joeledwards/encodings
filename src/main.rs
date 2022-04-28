fn main() {
    let unsigned_byte: u8 = u8::MAX;
    let signed_byte: i8 = -1;
    let unsigned_short: u16 = u16::MAX;
    let signed_short: i16 = -1;
    let unsigned_int: u32 = u32::MAX;
    let signed_int: i32 = -1;
    let unsigned_long: u64 = u64::MAX;
    let signed_long: i64 = -1;

    println!("u8 => {:x?}", unsigned_byte);
    println!("i8 => {:x?}", signed_byte);
    println!("u16 => {:x?}", unsigned_short);
    println!("i16 => {:x?}", signed_short);
    println!("u32 => {:x?}", unsigned_int);
    println!("i32 => {:x?}", signed_int);
    println!("u64 => {:x?}", unsigned_long);
    println!("i64 => {:x?}", signed_long);

    // Two's compliment

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
}
