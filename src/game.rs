use crate::levels::level::{Level, LevelConfig};
use crate::player::Player;
use crate::view::screen::clear_display;
use crate::view::{screen, text};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct GameText {
    intro: String,
    levels: Vec<LevelConfig>,
    secret_code: String,
}

pub struct Game {
    player: Player,
    levels: Vec<Level>,
    secret_code: String,
}

impl Game {
    pub fn new() -> Self {
        let player = Player::new();
        let config = Self::load_config();
        let levels = config.levels.into_iter().map(Level::new).collect();
        let secret_code: String = config.secret_code;
        Self { player, levels, secret_code }
    }

    pub fn run(&self) {
        self.show_intro();
        for level in &self.levels {
            level.run();
            screen::continue_prompt();
            screen::display_loading();
        }
        self.conclusion();
    }

    fn show_intro(&self) {
        let config = Self::load_config();
        println!("{}", text::green_text(&config.intro));
        screen::continue_prompt();
        screen::display_loading();
    }

    fn load_config() -> GameText {
        let content = fs::read_to_string("resources/game_text.json")
            .expect("Failed to read game configuration");
        serde_json::from_str(&content).expect("Failed to parse game configuration")
    }

    fn conclusion(&self) {
        let success = format!(
            "\n\tCongratulations, {}! You have proven yourself worthy.",
            self.player.name()
        );
        println!("{}", text::green_text(&success));
        println!("\n\tThe code before you unlocks your prize -- and my freedom. May the light you carry guide you always.");
        println!("\n\n\t\t{}\n", text::yellow_text(&self.secret_code));
        screen::continue_prompt();
        clear_display();
        screen::display_title();
    }
}
