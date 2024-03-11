fn greet() {
    let a = "Hello, world!";
    let b = "Grüß Gott!";
    let c = "ハロー・ワールド";

    let regions = [a, b ,c];

    for region in regions.iter() {
            println!("{}", &region);
    }
}

fn main() {
    greet();
}
