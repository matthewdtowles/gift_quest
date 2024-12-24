mod view;
mod level;

use std::io::{self, Write};

use level::level_one;
use view::text::text;


fn choose_player() -> String {
    println!("{}", text::green_text("Who embarks on this journey?"));
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    loading_msg();
    let msg = format!("Ah! So your name is {}?", user_input.trim());
    if !confirm_input(&msg) {
        return choose_player();
    }
    user_input.trim().to_string()
}

fn continue_prompt() {
    println!("{}", text::red_text("Press ENTER to continue..."));
    let mut user_input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).unwrap();
}

fn confirm_input(msg: &str) -> bool {
    let msg = format!("{} (y/n)", msg);
    println!("{}", text::green_text(&msg));
    let mut user_input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).unwrap();
    match user_input.trim() {
        "y" | "Y" | "yes" | "Yes" => true,
        "n" | "N" | "no" | "No" => false,
        _ => {
            println!("{}", text::red_text("Please enter 'y' or 'n'"));
            confirm_input(&msg)
        }
    }
}

fn intro_msg(player: &String) {
    let intro = format!(
        r#"
    Greetings, {player}!

    I am a spirit, bound to this realm by an unfulfilled purpose. 

    Long ago, I sought to deliver a gift of immeasurable value (approximately $25 in today's currency).

    Alas, my time ended before I could complete my task. 

    Now, I can only guide another to finish what I could not. 

    Help me complete my quest and I will guide you through the trials that await you.
    "#,
        player = player
    );
    println!("{}", intro);
}

fn title_msg() {
    let banner = r#"
    * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
      _______  __   _______ .___________.     ______      __    __   _______     _______.___________.
     /  _____||  | |   ____||           |    /  __  \    |  |  |  | |   ____|   /       |           |
    |  |  __  |  | |  |__   `---|  |----`   |  |  |  |   |  |  |  | |  |__     |   (----`---|  |----`
    |  | |_ | |  | |   __|      |  |        |  |  |  |   |  |  |  | |   __|     \   \       |  |     
    |  |__| | |  | |  |         |  |        |  `--'  '--.|  `--'  | |  |____.----)   |      |  |     
     \______| |__| |__|         |__|         \_____\_____\\______/  |_______|_______/       |__|     

    * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
                                                                                                     "#;
    println!("{}", text::yellow_text(banner));
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn loading_msg() {
    clear_screen();
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!(".");
    std::thread::sleep(std::time::Duration::from_secs(1));
    clear_screen();
    println!("..");
    std::thread::sleep(std::time::Duration::from_secs(1));
    clear_screen();
    println!("...");
    std::thread::sleep(std::time::Duration::from_secs(1));
    clear_screen();
}

fn main() {
    clear_screen();
    loading_msg();
    title_msg();
    continue_prompt();
    clear_screen();
    let player = choose_player();
    loading_msg();
    intro_msg(&player);
    continue_prompt();
    loading_msg();
    level_one::run();
}
