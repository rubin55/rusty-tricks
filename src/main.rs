fn greet() {
    let a = "Hello, world!";
    let b = "Grüß Gott!";
    let c = "ハロー・ワールド";

    let regions = [a, b ,c];

    for region in regions.iter() {
            println!("{}", &region);
    }
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn main() {
    greet();
    println!("10 + 15 = {}", add(10, 15))
}
