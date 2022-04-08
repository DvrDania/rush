mod rush_core;

use rush_core::Hex;

pub fn start(args: Vec<String>) {
    if let 1 = args.len() {
        println!("Not enough arguments");
        return;
    };
    let color = match Hex::try_from(args[1].clone()) {
        Ok(c) => c,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    let _shades = color.make_shades();
}
