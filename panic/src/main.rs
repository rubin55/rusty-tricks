use std::env;

#[derive(Debug)]
enum Platform {
  Linux,
  Macos,
  Windows
}

impl Platform {
  fn parse(p: &str) -> Platform {
    let u = p.to_uppercase();
    match u.as_ref() {
      "LINUX" => Platform::Linux,
      "MACOS" => Platform::Macos,
      "WINDOWS" => Platform::Windows,
      _ => panic!("Unknown platform: {}", p)
    }
  }
}
fn main() {
  let args: Vec<String> = env::args().collect();
  let first = &args[1];
  let platform = Platform::parse(first);
  println!("Producing output for {:?}", platform);
}
