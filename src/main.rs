mod levels;
mod view;
mod player;

use levels::level;
use view::screen;
use player::Player;

fn intro_msg(_player: &String) {
    let intro = format!(
        r#"
    Greetings, {player}!

    I am a spirit, bound to this realm by an unfulfilled purpose.

    Long ago, I sought to deliver a gift of immeasurable value (approximately $25 in today's currency).

    Alas, my time ended before I could complete my task.

    Now, I can only guide another to finish what I could not.

    Help me complete my quest and I will guide you through the trials that await you.
    "#,
        player = _player
    );
    println!("{}", intro);
}

fn main() {
    screen::display_title();
    screen::clear_display();
    let player = Player::new();
    screen::display_loading();
    intro_msg(player.name());
    screen::continue_prompt();
    screen::display_loading();
    level::run();
}
