use std::{thread, time};

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn loading_screen() {
    for _ in 0..3 {
        print!(".");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!();
}

pub fn wait(seconds: u64) {
    let wait_time = time::Duration::from_secs(1);
    thread::sleep(wait_time);
}