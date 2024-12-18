use std::io::stdin;

enum Player {
    Annabella,
    Carolyn,
    Jessica,
    Kennon,
}

fn choose_player() {
    println!("Choose a player: ");
    println!("1. Annabella");
    println!("2. Carolyn");
    println!("3. Jessica");
    println!("4. Kennon");
    let mut user_input = String::new();
    let choice = stdin().read_line(&mut user_input).unwrap();
    match choice {
        1 => Player::Annabella,
        2 => Player::Carolyn,
        3 => Player::Jessica,
        4 => Player::Kennon,
        _ => Player::Annabella,
    };
}

fn title_msg() {
    let banner = r#"
      _______  __   _______ .___________.     ______      __    __   _______     _______.___________.
     /  _____||  | |   ____||           |    /  __  \    |  |  |  | |   ____|   /       |           |
    |  |  __  |  | |  |__   `---|  |----`   |  |  |  |   |  |  |  | |  |__     |   (----`---|  |----`
    |  | |_ | |  | |   __|      |  |        |  |  |  |   |  |  |  | |   __|     \   \       |  |     
    |  |__| | |  | |  |         |  |        |  `--'  '--.|  `--'  | |  |____.----)   |      |  |     
     \______| |__| |__|         |__|         \_____\_____\\______/  |_______|_______/       |__|     
                                                                                                     "#;
    println!("{}", banner);
}

fn main() {
    title_msg();
    // pause(); // pause the game

    // title_prompt(); // prompt the user to start the game
    choose_player();
}
