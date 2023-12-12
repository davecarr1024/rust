use minigrep;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = minigrep::config::Config::new(args).expect("failed to create config");
    if let Err(e) = minigrep::run(cfg) {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
