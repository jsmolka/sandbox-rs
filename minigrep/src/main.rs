use std::env;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Cannot parse arguments: {error}");
        std::process::exit(1);
    });

    if let Err(error) = minigrep::run(config) {
        eprintln!("Application error: {error}");
        std::process::exit(1);
    }
}
