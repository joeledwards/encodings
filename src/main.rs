mod bit_ops;
mod int_ranges;
mod hash8;
mod cipher8;
mod hexes;

fn main() {
    int_ranges::run();
    bit_ops::run();
    hexes::run();
    hash8::run();
    cipher8::run();
}
