use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            filepath: args[2].clone(),
        })
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
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
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
