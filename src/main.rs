use std::{env, process};

use bmt::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    eprintln!("Parsing swarm hash on file {}", config.filename);

    if let Err(e) = bmt::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
