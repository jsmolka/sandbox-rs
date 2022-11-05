use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next(); // Program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("expected query string"),
        };

        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("expected filepath"),
        };

        Ok(Config { query, filepath })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filepath)?;

    for line in search(&config.query, &content) {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
