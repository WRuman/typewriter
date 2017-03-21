use std::{thread, time};
use std::io::Write;

fn main() {
    let msg = "Computers are like any other machine. They're either a benefit or a hazard. If they're a benefit, it's not my problem";
    let split_sec = time::Duration::from_millis(90);
    let pause_sec = split_sec * 8;
    for c in msg.chars() {
        print!("{}", c);
        // stdout is normally line buffered, so explicit flush is needed for typing effect
        match std::io::stdout().flush() {
            Ok(val) => val,
            Err(_) => {
                println!("Could not flush stdout");
                break;
            },
        }
        match c {
            '.' => thread::sleep(pause_sec),
            _ => thread::sleep(split_sec),
        }
    }
}
