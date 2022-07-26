pub mod chunk;
pub mod file;
pub mod span;

use std::error::Error;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // skip first element as name of binary
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        Ok(Config { filename })
    }
}

pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    // let contents = fs::read(config.filename);
    Ok(())
}

const SEGMENT_SIZE: usize = 32;
const SEGMENT_PAIR_SIZE: usize = 2 * SEGMENT_SIZE;
const DEFAULT_SPAN_SIZE: u8 = 8; // bytes
const HASH_SIZE: usize = 32; // bytes

const DEFAULT_MAX_PAYLOAD_SIZE: usize = 4096; // bytes
const DEFAULT_MIN_PAYLOAD_SIZE: usize = 1;
