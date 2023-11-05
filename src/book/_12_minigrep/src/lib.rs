use std::{fs};
use crate::config::Config;

pub mod config;

pub fn minigrep(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let results = search(&config, &contents);
    print(results);
    Ok(())
}

fn search<'a>(config: &Config, contents: &'a str) -> Vec<&'a str> {
    if config.ignore_case {
        search_case_insensitive(&config.query, contents)
    } else {
        search_case_sensitive(&config.query, contents)
    }
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.as_str()))
        .collect()
}

fn print(results: Vec<&str>) {
    results.iter().for_each(|&r| println!("{r}"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn searches_case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(search_case_sensitive(query, contents), vec!["safe, fast, productive."]);
    }

    #[test]
    fn searches_case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(search_case_insensitive(query, contents), vec!["Rust:", "Trust me."]);
    }
}
