use rush;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    rush::start(&args);
}
