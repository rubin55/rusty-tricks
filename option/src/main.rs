fn main() {
  let non_empty_list: Vec<u8> = vec![1, 2, 3];
  println!("non_empty_list's last: {:?}", non_empty_list.last());

  let empty_list: Vec<u8> = vec![];
  println!("empty_list's last: {:?}", empty_list.last());
}
