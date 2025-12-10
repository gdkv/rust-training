extern crate serde;

use std::fs;
use std::io::Error;
use self::serde::{Deserialize};
#[derive(Deserialize, Debug)]
pub struct Config {
    pub some_var: i32,
    pub other_var: i32,
    pub count: i32,
    pub str: String,
    pub active: bool
}

impl Config {
    pub fn load() -> Result<Config, Error> {
        let text = fs::read_to_string("config.json");

        match text {
            Ok(t) => Ok(serde_json::from_str(&t).unwrap()),
            Err(_) => Err(Error::other("config.json could not be opened")),
        }
    }
}