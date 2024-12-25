use crate::view::{screen, text};
use std::io;

pub struct Player {
    name: String,
}

impl Player {
    pub fn new() -> Self {
        let name = Self::choose_name();
        Self { name }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    fn choose_name() -> String {
        println!("{}", text::green_text("Who embarks on this journey?"));
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let name = input.trim().to_string();
        screen::display_loading();
        let msg = format!("Ah! So your name is {}?", name);
        if !screen::confirm_input(&msg) {
            return Self::choose_name();
        }
        name
    }
}
