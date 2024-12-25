mod game;
mod levels;
mod player;
mod view;

use game::Game;
use view::screen;

fn main() {
    screen::display_title();
    screen::clear_display();
    let game = Game::new();
    game.run();
}
