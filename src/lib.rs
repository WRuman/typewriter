use std::{thread, time};
use std::io::{stdout, Write};

pub fn type_print(msg: &str, millis: u64) {
    let split_sec = time::Duration::from_millis(millis);
    let pause_sec = split_sec * 8;

    for c in msg.chars() {
        print!("{}", c);
        // stdout is normally line buffered, so explicit flush is needed for typing effect
        match stdout().flush() {
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
    println!();

}
