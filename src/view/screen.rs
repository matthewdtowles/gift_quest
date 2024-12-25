use crate::view::text;
use std::fs;
use std::io::{self, Write};

pub fn clear_display() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn confirm_input(msg: &str) -> bool {
    let msg = format!("{} (y/n)", msg);
    println!("{}", text::green_text(&msg));
    let mut user_input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).unwrap();
    match user_input.trim() {
        "y" | "Y" | "yes" | "Yes" => true,
        "n" | "N" | "no" | "No" => false,
        _ => {
            println!("{}", text::red_text("\n\t\tPlease enter 'y' or 'n'"));
            confirm_input(&msg)
        }
    }
}

pub fn continue_prompt() {
    println!("{}", text::red_text("\n\t\tPress ENTER to continue..."));
    let mut user_input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).unwrap();
}

pub fn display_loading() {
    clear_display();
    for _ in 0..3 {
        print!(".");
        std::io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    clear_display();
}

pub fn display_title() {
    clear_display();
    display_loading();
    let banner = fs::read_to_string("resources/title.txt").expect("Unable to read file");
    println!("{}", text::yellow_text(&banner));
    continue_prompt();
}
