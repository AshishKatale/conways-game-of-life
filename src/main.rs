mod gameloop;
mod gamestate;

use crate::gameloop::GameLoop;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut game_of_life = GameLoop::new();
    if args.len() > 1 {
        game_of_life.init_state_from_csv(&args[1]);
    }
    game_of_life.run();
}
