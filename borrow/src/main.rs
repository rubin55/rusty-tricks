struct Person {
    name: String
}

fn congrats(p: &Person) {
    println!("Congratulations, {}!!!", p.name)
}

fn main() {
    let p = Person {
        name: String::from("Jake"),
    };

    congrats(&p);
    println!("Can still use p here: {}", p.name);
}
