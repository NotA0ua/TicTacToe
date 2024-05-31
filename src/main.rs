mod game;

use game::build_game;
use game::Game;

fn main() {
    let mut game: Game = build_game();

    game.run()
}
