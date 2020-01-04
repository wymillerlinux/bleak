// external crates getting imported
extern crate reqwest;
extern crate select;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

// std lib imports
use std::{thread, time};

// actual third party library being imported
use serde::{Deserialize, Serialize};
use serde_json::Result;

// local files that need to be imported
mod app;
mod config;
mod generate;
mod led;

fn main() {
    let configuration = config::init_config();
    println!("{:?}", configuration);

    loop {
        let app_text = configuration.get_tv_status();
        let power_text = configuration.get_power_status();

        let activeapp = app::match_to_app(app_text);
        let tvpower = app::match_to_power_status(power_text);

        match activeapp {
            app::ActiveApp::Roku => println!("The lights are light purple!"),
            app::ActiveApp::Netflix => println!("The lights are red!"),
            app::ActiveApp::Hulu => println!("The lights are green!"),
            app::ActiveApp::AmazonPrime => println!("The light are light blue!"),
            app::ActiveApp::Spotify => println!("The lights are light green!"),
            _ => println!("Oops!"),
        }

        match tvpower {
            app::TVPower::On => println!("TV is on!"),
            app::TVPower::Off => println!("TV is off!"),
            _ => println!("We don't know what the power status of the TV is..."),
        }

        let sec = time::Duration::from_secs(3);
        thread::sleep(sec);
    }
}
