use std::{env, process::exit};

use unicodificator::Config;

const USAGE_MSG: &str = "{photo} {size} {filter} [contrast_modifier]

Allowed Filter Types:
    near | nearest => FilterType::Nearest
    tri | triangle => FilterType::Triangle
    cat | catmull => FilterType::CatmullRom
    gaus | gaussian => FilterType::Gaussian
    lan | lanczos3 => FilterType::Lanczos3";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "help" {
        println!("Usage: {} {}", args[0], USAGE_MSG);
        exit(0);
    } else if args.len() < 4 {
        eprintln!("ERROR: Usage: {} {}", args[0], USAGE_MSG);
        exit(1);
    }

    let config = Config::new(
        &args[1],
        &args[2],
        &args[3],
        args.get(4).unwrap_or(&String::from("0.0")),
    )
    .unwrap();
    unicodificator::run(config).unwrap();
}
