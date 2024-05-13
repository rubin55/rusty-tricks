use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
struct Person {
  name: String
}

fn main() {
  // Using json! macro.
  let first = Person::deserialize(json!({ "name": "Margaret Hamilton" }));

  // Don't handle error, everything but Ok -> unimplemented. Note: could also first.unwrap_or_default().
  let first_inner = match first {
    Ok(x) => x,
    _ => unimplemented!()
  };

  println!("First: {:?}", first_inner.name);

  // Using from_str.
  let second = serde_json::from_str::<Person>(r#"{ "name": "Johnny Example", }"#);

  // Handle error, print "Unknown" instead.  Note: could also second.unwrap_or_default().
  let second_inner = match second {
    Ok(x) => x,
    Err(_) => Person { name: String::from("Unknown")}
  };

  println!("Second: {:?}", second_inner.name);
}
