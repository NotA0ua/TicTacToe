mod game;

use game::build_game;
use game::Game;
use game::Forms;

fn main() {
    let mut game: Game = build_game(Forms::Circle);

    game.run()
}
