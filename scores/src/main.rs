struct Player {
  score: i32
}

impl Player {
  fn set_score(&mut self, new_score: i32) {
    self.score = new_score;
  }

  fn score (&self) -> i32 {
    self.score
  }

  fn new() -> Self {
    Player { score: 0 }
  }
}



fn main() {
  let mut player = Player::new();
  let old_score = player.score();
  player.set_score(old_score + 1);
  println!("Score: {}", player.score());
}
