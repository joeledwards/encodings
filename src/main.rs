mod bit_ops;
mod int_ranges;
mod hash8;
mod cipher8;

fn main() {
    int_ranges::run();
    bit_ops::run();
    hash8::run();
    cipher8::run();
}
