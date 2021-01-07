mod engine;
use engine::game_state::GameState;

fn main() {
    println!("Hello, world!");
    let x: GameState = GameState::new();
    println!("{}", x);
}
