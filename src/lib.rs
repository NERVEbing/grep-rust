use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::{env, io};

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("query is nil"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("filename is nil"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_file(&config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };
    for line in result.iter() {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| line.trim())
        .collect()
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .map(|line| line.trim())
        .collect()
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Apache";
        let filename = "./LICENSE";
        let contents = read_file(filename).unwrap();
        let results = search(query, contents.as_str());
        for line in results.iter() {
            println!("matching line : {}", line);
        }
        assert!(results.len() > 0)
    }
}
