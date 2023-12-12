use std::env;

pub struct Config {
    pub pattern: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        const USAGE: &str = "Invalid args. Usage: minigrep <pattern> <filename>";
        Ok(Config {
            pattern: args.get(1).ok_or(USAGE)?.clone(),
            filename: args.get(2).ok_or(USAGE)?.clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}
