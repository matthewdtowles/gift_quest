use std::io;

pub fn run() {
    title();
    story();
    riddle();
    prompt_answer();
}

fn title() {
    let title = r#"
    Level One: The Unlit Path
--------------------------------

    "#;
    println!("{}", title);
}

fn story() {
    let msg = r#"
    A soft, ethereal voice slowly fills the air, a voice that only you can hear. 

    It seems to come from everywhere at once, like a whisper carried by the wind. 

    The voice speaks:

        "I am the spirit of knowledge, bound to the darkness of ignorance.
        In this world of shadows, the truth and knowledge are the light that guides the way.
        But this truth has been obscured, and only those who are worthy may bring forth the light.
        I sense something in you.
        Solve my riddle, and I shall grant you the torch to illuminate your way."
    "#;
    println!("{}", msg);
}

fn riddle() {
    let msg: &str = r#"
        I am always ahead of you, yet never seen,
        A guide through the shadows, but hidden in between.
        You cannot touch me, but I light your way,
        I am what you seek, but never fully display.
    "#;
    println!("{}", msg);
}

fn prompt_answer() {
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    let answer = answer.trim();
    if answer == "truth" || answer == "Truth" {
        println!("The voice speaks: 'You have answered correctly. Here is the torch.'");
    } else {
        println!("The voice speaks: 'You must try again, for we both seek not falsehood.'");
        prompt_answer();
    }
}
