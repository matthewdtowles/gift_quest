pub fn red_text(text: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", text)
}

pub fn green_text(text: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", text)
}

pub fn yellow_text(text: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", text)
}

pub fn blue_text(text: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", text)
}

pub fn cyan_text(text: &str) -> String {
    format!("\x1b[36m{}\x1b[0m", text)
}
