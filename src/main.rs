use grep_rust::run;
use grep_rust::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("error to parse config: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("error to run: {}", e);
        process::exit(1);
    }
}
