use std::env;
use std::process;
use rust_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else( |err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rust_minigrep::run(config) {
        eprint!("Application error: {}", e);
        process::exit(1);
    }
}
