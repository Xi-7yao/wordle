mod lib ;
mod words;

use lib::game::Game;

fn main() {


    let mut game = Game::new() ;
    game.game_run();
}
