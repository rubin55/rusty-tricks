use std::convert::TryInto;

fn main() {
    let a: i8 = 42;
    let b: i16 = 1025;

    let c: i32 = 324985;
    let d: i64 = c.try_into().unwrap();
    let e: i64 = 23482302859204;

    if (a as i16) < b {
        println! ("a: {} is less than b: {}", a, b)
    }

    if d < e {
        println! ("d: {} is less than e: {}", d, e)
    }
}
