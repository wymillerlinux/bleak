use std::fs::File;
use std::io::Read;
use std::path::Path;

use select::document::Document;
use select::predicate::Name;
use serde::{Serialize, Deserialize};

// config structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub ipaddr: String,
    pub port: i32,
    pub tv: String,
    pub active_app: String,
    pub power_status: String,
}

impl Configuration {
    pub fn is_change_app(&self) -> bool {
        self.active_app == self.get_app_status()
    }

    pub fn change_active_app(&mut self, app_text: &String) {
        self.active_app = app_text.to_string();
    }

    pub fn change_power(&mut self, power_text: &String) {
        self.power_status = power_text.to_string();
    }

    pub fn get_app_status(&self) -> String {
        let request = format!(
            "http://{ipaddr}:{port}/query/active-app",
            ipaddr = self.ipaddr,
            port = self.port
        );

        let response = get_request(&request);
        match response {
            Ok(res) =>  {
                let document = Document::from_read(res)
                    .unwrap();
                let next = document.find(Name("app"))
                    .next()
                    .unwrap();
                next.text()
                    .to_string()
            }
            Err(_) => "_".to_string()         
        }
    }

    pub fn get_power_status(&self) -> String {
        let request = format!(
            "http://{ipaddr}:{port}/query/device-info",
            ipaddr = self.ipaddr,
            port = self.port
        );
        
        let response = get_request(&request);
        match response {
            Ok(res) =>  {
                let document = Document::from_read(res)
                    .unwrap();
                let next = document.find(Name("power-mode"))
                    .next()
                    .unwrap();
                next.text()
                    .to_string()
            }
            Err(_) => "_".to_string()         
        }
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

pub fn get_request(request: &String) -> Result<reqwest::Response, reqwest::Error> {
    let response = reqwest::get(request);
    response
}