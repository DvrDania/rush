mod rush_core;

use rush_core::Hex;

pub fn run(args: Vec<String>) {
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
    let shades = color.make_shades();
    println!("{{");
    for (i, shade) in shades.iter().enumerate() {
        if i == 0 {
            println!("      50: \"{shade}\",");
        } else {
            println!("      {}: \"{shade}\",", i * 100);
        }
    }
    println!("}}");
}
