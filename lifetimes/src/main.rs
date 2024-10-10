#[allow(clippy::needless_lifetimes)]
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn main() {
    let a = 10;
    let b = 20;
    let x = add_with_lifetimes(&a, &b);
    println!("Result is: {}", x);
}
