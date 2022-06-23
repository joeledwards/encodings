mod bit_ops;
mod int_ranges;
mod hash8;
mod cipher8;
mod hexes;
mod b32;
mod b64;

fn main() {
    int_ranges::run();
    bit_ops::run();
    hexes::run();
    b32::run();
    b64::run();
    hash8::run();
    cipher8::run();
}
