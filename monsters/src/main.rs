struct Stats {
    hp: u8,
    sp: u8,
}

struct Monster {
    stats: Stats,
    friends: Vec<Friend>
}

struct Friend {
    loyalty: u8
}

impl Monster {
    fn final_breath(&mut self) {
        if let Some(friend) = self.friends.first() {
            self.stats.heal(friend.loyalty);
        }
    }
}

impl Stats {
    fn heal(&mut self, amount: u8) {
        self.hp += amount;
        self.sp -= amount;
        println!("Healing for {}", amount);
    }
}

fn main() {
    let mut _stats = Stats { hp: 20, sp: 12 };
    let mut _friends = vec![Friend { loyalty: 4 }];
    let mut _monster = Monster { stats: _stats, friends: _friends };
    _monster.final_breath();
    println!("Hello, monster!");
}
