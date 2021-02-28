use std::io;

mod engine;
use engine::game_state::GameState;

fn main() {
    println!("Hello, world!");
    let x: GameState = GameState::new();
    println!("{}", x);

    let mut line: String = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).unwrap();

    let y = x.move_code(line.as_str()).unwrap();
    println!("{}", y);
}
