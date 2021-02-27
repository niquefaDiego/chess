pub struct GameState {
  flags: i8, // can castle e1, can castle e8
  last_move: i8,
  board: [[u8; 8]; 8]
}


impl GameState {
  pub fn new() -> GameState {
    let mut board = [[0; 8]; 8];
    for i in 0..8 {
      board[1][i] = 11;
      board[6][i] = 12;
    }
    board[0][0] = 5;
    board[0][1] = 9;
    board[0][2] = 7;
    board[0][3] = 3;
    board[0][4] = 1;
    board[0][5] = 7;
    board[0][6] = 9;
    board[0][7] = 5;

    board[7][0] = 6;
    board[7][1] = 10;
    board[7][2] = 8;
    board[7][3] = 4;
    board[7][4] = 2;
    board[7][5] = 8;
    board[7][6] = 10;
    board[7][7] = 6;
    
    GameState {
      flags: 0,
      last_move: 0,
      board
    }
  }
}

impl std::fmt::Display for GameState {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let pieces = ['.', 'K', 'k', 'Q', 'q', 'R', 'r', 'B', 'b', 'N', 'n', 'P', 'p'];
    let white_view = true;
    let left_margin = "                                ";
    writeln!(f, "flags = {}, last_move = {}", self.flags, self.last_move).unwrap();
    writeln!(f, "").unwrap();
    if white_view {
      writeln!(f, "{}  A B C D E F G H", left_margin).unwrap();
    } else {
      writeln!(f, "{}  H G F E D C B A", left_margin).unwrap();
    }
    writeln!(f, "{} +-+-+-+-+-+-+-+-+", left_margin).unwrap();
    for i in 0..8 {
      write!(f, "{}{}|", left_margin, if white_view { 8 - i } else { i+1 } ).unwrap();
      for j in 0..8 {
        if white_view {
          write!(f, "{}|", pieces[self.board[7-i][j] as usize]).unwrap();
        } else {
          write!(f, "{}|", pieces[self.board[i][7-j] as usize]).unwrap();
        }
      }
      write!(f, "\n").unwrap();
      writeln!(f, "{} +-+-+-+-+-+-+-+-+", left_margin).unwrap();
    }
    Ok(())
  }
}
