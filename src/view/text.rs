pub mod text {

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

    pub fn magenta_text(text: &str) -> String {
        format!("\x1b[35m{}\x1b[0m", text)
    }

    pub fn bold_text(text: &str) -> String {
        format!("\x1b[1m{}\x1b[0m", text)
    }

    pub fn underline_text(text: &str) -> String {
        format!("\x1b[4m{}\x1b[0m", text)
    }

    pub fn italic_text(text: &str) -> String {
        format!("\x1b[3m{}\x1b[0m", text)
    }

    pub fn strikethrough_text(text: &str) -> String {
        format!("\x1b[9m{}\x1b[0m", text)
    }

    pub fn blink_text(text: &str) -> String {
        format!("\x1b[5m{}\x1b[0m", text)
    }

    pub fn reverse_text(text: &str) -> String {
        format!("\x1b[7m{}\x1b[0m", text)
    }

    pub fn invisible_text(text: &str) -> String {
        format!("\x1b[8m{}\x1b[0m", text)
    }

    pub fn black_text(text: &str) -> String {
        format!("\x1b[30m{}\x1b[0m", text)
    }

    pub fn white_text(text: &str) -> String {
        format!("\x1b[37m{}\x1b[0m", text)
    }

    pub fn bg_black_text(text: &str) -> String {
        format!("\x1b[40m{}\x1b[0m", text)
    }

    pub fn bg_white_text(text: &str) -> String {
        format!("\x1b[47m{}\x1b[0m", text)
    }

    pub fn bg_red_text(text: &str) -> String {
        format!("\x1b[41m{}\x1b[0m", text)
    }

    pub fn bg_green_text(text: &str) -> String {
        format!("\x1b[42m{}\x1b[0m", text)
    }

    pub fn bg_yellow_text(text: &str) -> String {
        format!("\x1b[43m{}\x1b[0m", text)
    }

    pub fn sanitize_text(text: &str) -> String {
        text.replace("\n", "")
            .replace("\r", "")
            .replace("\t", "")
            .replace("\x1b", "")
            .replace("[", "")
            .replace("m", "")
    }
}
