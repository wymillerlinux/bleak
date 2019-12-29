// external crates getting imported
extern crate ureq;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate xml;

// standard library stuff
use std::fs::File;
use std::io::{BufReader, Read};

// actual third party library being imported
use serde::{Deserialize, Serialize};
use serde_json::Result;
use xml::reader::{EventReader, XmlEvent};

// local files that need to be imported
mod app;
mod config;
mod generate;

fn main() {
    let configuration = config::init_config();
    println!("{:?}", configuration.ipaddr);
}
