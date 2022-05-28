use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_file(&config.filename)?;
    let result = search(&config.query, contents.as_str());

    for line in result.iter() {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim())
        }
    }

    results
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
