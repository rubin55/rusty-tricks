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

fn add_five_to_last(n: &[i32]) -> Option<i32> {
  let num = n.last()?;
  Some(num + 5)
}


fn main() {
    let msg = save_status("Tumdidum");
    println!("{:?}", msg);

    let non_empty_list = vec![1,2,3];
    println!("add_five_to_last on non_empty_list: {:?}", add_five_to_last(&non_empty_list));

    let empty_list: Vec<i32> = vec![];
    println!("add_five_to_last on empty_list: {:?}", add_five_to_last(&empty_list));
}
