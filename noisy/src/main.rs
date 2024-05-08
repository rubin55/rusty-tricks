 struct Noisy {
     id: u32
 }

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("Noisily going out of scope: {}", self.id)
    }
}
fn main() {
    let _n1 = Noisy { id: 1};
    let _n2 = Noisy { id: 2};
    let _n3 = Noisy { id: 3};

    println!("Main ends here.")
}
