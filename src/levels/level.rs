use std::io;

pub fn run() {
    println!("{}", title("Level One: The Unlit Path"));
    println!("{}", story());
    println!("{}", riddle());
    prompt_answer();
}

fn title(msg: &str) -> String {
    let underline = "-".repeat(msg.len() + 8);
    format!("\n    {}\n{}\n", msg, underline)
}

fn story() -> String {
    String::from(r#"
    A soft, ethereal voice slowly fills the air, a voice that only you can hear. 

    It seems to come from everywhere at once, like a whisper carried by the wind. 

    The voice speaks:

        "I am the spirit of knowledge, bound to the darkness of ignorance.
        In this world of shadows, the truth and knowledge are the light that guides the way.
        But this truth has been obscured, and only those who are worthy may bring forth the light.
        I sense something in you. Prove you are worthy and I shall grant you the torch to illuminate your way."
    "#)
}

fn riddle() -> String {
    String::from(r#"
        I am forever in front, yet never out of reach,
        A beacon in darkness, where clarity must teach,
        I am sought by the wise, but shunned by the fool,
        A treasure to the seeker, a weapon to the cruel,
        Eternal and pure, though shadows I show,
        I hold no shape, yet I shape what you know.

        What am I?
    "#)
}

fn prompt_answer() {
    println!("Enter your answer: ");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    let answer = answer.trim().to_lowercase();
    if answer == "truth" {
        println!("The voice speaks: 'Correct. Let this torch be your truth.'");
    } else {
        println!("The voice speaks: 'Try again, for we both seek not falsehood.'");
        prompt_answer();
    }
}
