#[derive(Debug)]
struct CarPool {
  passengers: Vec<String>
}

impl CarPool {
  fn pickup(&mut self, name: String) {
    self.passengers.push(name);
  }
}

fn main() {
    let mut carpool = CarPool { passengers: vec![] };
    println!("carpool contents: {:?}", carpool);

    carpool.pickup(String::from("Jake"));
    println!("carpool contents: {:?}", carpool);

    carpool.pickup(String::from("Carol"));
    println!("carpool contents: {:?}", carpool);
}
