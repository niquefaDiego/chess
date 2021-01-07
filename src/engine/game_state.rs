pub struct GameState {
  flags: i8, // can castle e1, can castle e8
  last_move: i8,
  board: [[u8; 8]; 8]
}


impl GameState {
  pub fn new() -> GameState {
    GameState {
      flags: 0,
      last_move: 0,
      board: [[0; 8]; 8]
    }
  }
}

impl std::fmt::Display for GameState {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    writeln!(f, "flags = {}", self.flags);
    writeln!(f, "last_move = {}", self.last_move);
    for i in 0..8 {
      for j in 0..8 {
        write!(f, "{}", self.board[i][j]);
      }
      write!(f, "\n");
    }
    Ok(())
  }
}
