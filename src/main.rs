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

fn main() {
    let configuration = config::init_config();
    println!("{:?}", configuration);

    loop {
        let text = config::get_tv_status(&configuration);

        let activeapp = app::match_to_app(text);

        match activeapp {
            app::ActiveApp::Roku => println!("The lights are light purple!"),
            app::ActiveApp::Netflix => println!("The lights are red!"),
            app::ActiveApp::Hulu => println!("The lights are green!"),
            app::ActiveApp::AmazonPrime => println!("The light are light blue!"),
            app::ActiveApp::Spotify => println!("The lights are light green!"),
            _ => println!("Oops!"),
        }

        let sec = time::Duration::from_secs(1);
        thread::sleep(sec);
        //println!("{:?}", text as str);
    }
}
