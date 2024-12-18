use std::io::stdin;

// TODO: make players have number associated for easier dynamic selection in runtime
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
        _ => Player::Annabella, //TODO: need to change this so that we keep calling the function until a
                                // valid choice is made
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
    // TODO: PAUSE until key event
    // TODO: prompt the user to start the game
    // TODO: when user key detected, continue:
    choose_player();
    // TODO: save player choice as player
    // TODO: start the game
    // TODO: display the game board
    // TODO: prompt the user to make a move
    // TODO: step 1 - 14
    // TODO: check for winner
    // TODO: if winner, display winner
    // TODO: if no winner, display next step
    // TODO: add in players throughout the story
}
