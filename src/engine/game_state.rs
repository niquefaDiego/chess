const ASCII_A: u8 = 65;
const ASCII_H: u8 = ASCII_A + 7;
const ASCII_1: u8 = 49;
const ASCII_8: u8 = ASCII_1 + 7;

pub struct GameState {
  flags: u8, // can castle e1, can castle e8
  last_move: i16,
  board: [[u8; 8]; 8],
  turn: u8 // 1 for white, 0 for black
}


impl GameState {
  pub fn new() -> GameState {
    let mut board = [[0; 8]; 8];
    for i in 0..8 {
      board[i][1] = 11;
      board[i][6] = 12;
    }
    board[0][0] = 5;
    board[1][0] = 9;
    board[2][0] = 7;
    board[3][0] = 3;
    board[4][0] = 1;
    board[5][0] = 7;
    board[6][0] = 9;
    board[7][0] = 5;

    board[0][7] = 6;
    board[1][7] = 10;
    board[2][7] = 8;
    board[3][7] = 4;
    board[4][7] = 2;
    board[5][7] = 8;
    board[6][7] = 10;
    board[7][7] = 6;
    
    GameState {
      flags: 0,
      last_move: 0,
      board,
      turn: 1
    }
  }

  fn move_coords(&self, fi: u8, fj: u8, ti: u8, tj: u8) -> GameState {
    println!("({},{}) -> ({},{})", fi,fj,ti, tj );
    let mut board = self.board.clone();
    board[ti as usize][tj as usize] = board[fi as usize][fj as usize];
    board[fi as usize][fj as usize] = 0;
    let r = GameState {
      flags: self.flags,
      last_move: ((fi as i16) << 9) | ((fj as i16) << 6) | ((ti as i16) << 3) | (tj as i16),
      board,
      turn: self.turn ^ 1
    };
    r
  }

  pub fn move_code(&self, code: &str) -> Result<GameState, &'static str> {
    let code_str: String = code.to_ascii_uppercase().trim().split_whitespace().collect();
    let bytes: &[u8] = code_str.as_bytes();
    println!("Upper: {:?}", bytes);
    if bytes.len() == 2 {
      if ASCII_A > bytes[0] || bytes[0] > ASCII_H {
        return Err("E_INVALID_MOVE")
      }
      if ASCII_1 > bytes[0] || bytes[1] > ASCII_8 {
        return Err("E_INVALID_MOVE")
      }
      let ti = bytes[0] - ASCII_A;
      let tj = bytes[1] - ASCII_1;
      let d: i8 = if self.turn == 1 { -1 } else { 1 };
      let si = ti;
      let sj = (tj as i8 + d) as u8;
      // TODO:
      println!("(i,j) = ({},{})", ti,tj );
      return Ok(self.move_coords(si, sj, ti, tj));
    }
    return Err("E_INVALID_MOVE");
  }
}

impl std::fmt::Display for GameState {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    println!("raw_board = [");
    for i in 0..8 {
      println!("{:?}", self.board[i]);
    }
    println!("]");
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
          write!(f, "{}|", pieces[self.board[j][7-i] as usize]).unwrap();
        } else {
          write!(f, "{}|", pieces[self.board[7-j][i] as usize]).unwrap();
        }
      }
      write!(f, "\n").unwrap();
      writeln!(f, "{} +-+-+-+-+-+-+-+-+", left_margin).unwrap();
    }
    Ok(())
  }
}
