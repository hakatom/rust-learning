use std::fs;

use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;
    println!("with text:\n{contents}");
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        Ok(Config {query, filepath})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
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
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}