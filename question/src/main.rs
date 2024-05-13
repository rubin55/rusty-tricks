use std::time::Instant;


fn save_status(text: &str) -> Result<i64, &'static str> {
  if text.len() > 200 {
    return Err("Status text is too long")
  }


  let record = save_to_database(text)?;

  // Note the above is the same as:
  // let record = try!(save_to_database(text));

  // Note: both of the above is also the same as:
  // let record= match save_to_database(text) {
  //   Err(err) => return Err(err),
  //   Ok(rec) => rec
  // };
  println!("id: {}, text: {}, created_at: {:?}", record.id, record.text, record.created_at);
  Ok(record.id)
}

fn save_to_database(text: &str) -> Result<StatusRecord, &'static str> {
  Ok(StatusRecord { id: 23, text: String::from(text), created_at: Instant::now()})
}

struct StatusRecord {
  id: i64,
  text: String,
  created_at: Instant
}

fn main() {
    let msg = save_status("Tumdidum");
    println!("{:?}", msg);
}
