use std::{env, process};

use rust_cli::{Config, run};

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_path);

    if let Err(e) = run(config) {
        eprintln!("{e}");
        process::exit(1);
    };
}
