extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::future::Future;

use reqwest::Client;
use select::document::Document;
use select::predicate::Name;
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

impl Configuration {
    pub fn is_change_app(&self) -> bool {
        true
    }

    pub fn change_active_app(&mut self) {
        // stuff happens here
    }

    pub fn get_tv_status(&self) -> String {
        let request = format!(
            "http://{ipaddr}:{port}/query/active-app",
            ipaddr = self.ipaddr,
            port = self.port
        );

        let response = reqwest::get(&request).unwrap();
        let document = Document::from_read(response).unwrap();
        let next = document.find(Name("app")).next().unwrap();

        next.text().to_string()
    }

    pub fn get_power_status(&self) -> String {
        let request = format!(
            "http://{ipaddr}:{port}/query/device-info",
            ipaddr = self.ipaddr,
            port = self.port
        );
        
        let response = reqwest::get(&request).unwrap();
        let document = Document::from_read(response).unwrap();
        let next = document.find(Name("power-mode")).next().unwrap();

        next.text().to_string()
    }

}

// grab the config
pub fn init_config() -> Configuration {
    let mut file = File::open(&Path::new("config.json")).expect("Couldn't grab JSON!");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Couldn't read file!");

    let config: Configuration = serde_json::from_str(&data).expect("Couldn't parse JSON!");

    config
}
