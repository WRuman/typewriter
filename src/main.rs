extern crate typewriter;

use typewriter::type_print;

fn main() {
    let msg = "Computers are like any other machine. They're either a benefit or a hazard. If they're a benefit, it's not my problem";
    type_print(msg, 90);
}
