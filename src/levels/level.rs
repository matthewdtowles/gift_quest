use crate::view::text;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LevelConfig {
    pub id: u32,
    pub title: String,
    pub story: String,
    pub riddle: String,
    pub answer: String,
    pub success_message: String,
    pub failure_message: String,
}

pub struct Level {
    config: LevelConfig,
}

impl Level {
    pub fn new(config: LevelConfig) -> Self {
        Self { config }
    }

    pub fn run(&self) {
        println!("{}", text::blue_text(&self.format_title()));
        println!("{}", self.config.story);
        println!("{}", text::cyan_text(&self.config.riddle));
        self.prompt_answer();
    }

    fn format_title(&self) -> String {
        let msg = format!("Level {}: {}", self.config.id, self.config.title);
        let underline = "-".repeat(msg.len() + 8);
        format!("\n    {}\n{}\n", msg, underline)
    }

    fn prompt_answer(&self) {
        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        if answer.trim().to_lowercase() == self.config.answer {
            println!("{}", text::green_text(&self.config.success_message));
        } else {
            println!("{}", text::red_text(&self.config.failure_message));
            self.prompt_answer();
        }
    }
}
