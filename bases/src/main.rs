fn main() {
    let a = 0b11;
    let b = 0o36;
    let c = 0x12C;

    println!("base  2: {:b} {:b} {:b}", a, b, c);
    println!("base  8: {:o} {:o} {:o}", a, b, c);
    println!("base 10: {} {} {}", a, b, c);
    println!("base 16: {:x} {:x} {:x}", a, b, c);
}
