mod level;
mod view;

use level::level_one;
use std::io;
use view::{screen, text};

fn choose_player() -> String {
    println!("{}", text::green_text("Who embarks on this journey?"));
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    screen::display_loading();
    let msg = format!("Ah! So your name is {}?", user_input.trim());
    if !screen::confirm_input(&msg) {
        return choose_player();
    }
    user_input.trim().to_string()
}

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
    let player = choose_player();
    screen::display_loading();
    intro_msg(&player);
    screen::continue_prompt();
    screen::display_loading();
    level_one::run();
}
