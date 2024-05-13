use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
struct Person {
  name: String
}

fn main() {
  // Using json! macro.
  let first = Person::deserialize(json!({ "name": "Margaret Hamilton" }));
  println!("First: {:?}", first);

  // Using from_str.
  let second = serde_json::from_str::<Person>(r#"{ "name": "Johnny Example", }"#);
  println!("Sedcond: {:?}", second);
}
