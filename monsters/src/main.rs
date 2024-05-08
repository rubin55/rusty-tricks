struct Monster {
    hp: u8,
    sp: u8,
    friends: Vec<Friend>
}

struct Friend {
    loyalty: u8
}

impl Monster {
    fn final_breath(&mut self) {
        if let Some(friend) = self.friends.first() {
            self.hp += friend.loyalty;
            self.sp -= friend.loyalty;
            println!("Haling for {}", friend.loyalty);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
