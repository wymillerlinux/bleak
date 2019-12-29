extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use serde::Deserialize;
use serde_json::Result;

// config structure
#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub ipaddr: String,
    pub port: i32,
    pub tv: String,
    pub active_app: String,
}

// grab the config
pub fn init_config() -> Configuration {
    let mut file = File::open(&Path::new("config.json")).expect("Couldn't grab JSON!");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Poop");

    let config: Configuration = serde_json::from_str(&data).expect("Couldn't parse JSON!");

    config
}
