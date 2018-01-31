extern crate typewriter;

use typewriter::type_print;
use std::env;

static MSG: &str = "Computers are like any other machine. They're either a benefit or a hazard. If they're a benefit, it's not my problem";

fn main() {
    let args: Vec<String> = env::args().collect();
    let output = match args.len() {
        n if n >= 2 => args[1].as_str(),
        _ => MSG
    };
    type_print(output, 90);
}
