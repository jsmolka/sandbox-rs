use std::env;
use std::error::Error;

use minigrep::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Cannot parse arguments: {error}");
        std::process::exit(1);
    });

    minigrep::run(config)
}
