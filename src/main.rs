// external crates getting imported
extern crate reqwest;
extern crate select;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

// std lib imports
use std::{thread, time};

// actual third party library being imported
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use smart_leds::{RGB8, SmartLedsWrite, brightness};
use ws2812_spi::Ws2812;

// local files that need to be imported
mod app;
mod config;
mod generate;

const NUM_LEDS: usize = 150;

fn main() {
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 3_000_000, Mode::Mode0).unwrap();
    let mut ws = Ws2812::new(spi);
    let mut configuration = config::init_config();
    let mut is_headless: bool = false;

    loop {
        println!("{:?}", configuration);

            
        let power_text = configuration.get_power_status();
        configuration.change_power(&power_text);
        let tvpower = app::match_to_power_status(power_text);

        match tvpower {
            app::TVPower::Off => {
                if is_headless == false {
                    is_headless = true;
                    let color = RGB8::new(0, 0, 0);
                    let mut data = [RGB8::default(); NUM_LEDS];

                    for i in 0..NUM_LEDS {
                        data[i] = color;
                    }
                    
                    ws.write(brightness(data.iter().cloned(), 32)).unwrap();
                }
            },
            app::TVPower::On => {
                if false == configuration.is_change_app() {
                    is_headless = false;

                    let app_text = configuration.get_app_status();
                    configuration.change_active_app(&app_text);
                    let activeapp = app::match_to_app(app_text);

                    match activeapp {
                        app::ActiveApp::Roku => {
                            let data = change_color(&255, &0, &255);
                            ws.write(brightness(data.iter().cloned(), 10)).unwrap();
                        },
                        app::ActiveApp::Hulu => {
                            let data = change_color(&51, &255, &85);
                            ws.write(brightness(data.iter().cloned(), 32)).unwrap();
                        },
                        app::ActiveApp::Netflix => {
                            let data = change_color(&255, &77, &77);
                            ws.write(brightness(data.iter().cloned(), 32)).unwrap();
                        },
                        app::ActiveApp::AmazonPrime => println!("The light are light blue!"),
                        app::ActiveApp::Spotify => {
                            let data = change_color(&51, &255, &85);
                            ws.write(brightness(data.iter().cloned(), 32)).unwrap();
                        },
                        app::ActiveApp::Plex => {
                            let data = change_color(&255, &187, &51);
                            ws.write(brightness(data.iter().cloned(), 32)).unwrap();               
                        },
                        _ => println!("We don't know what app is running right now..."),
                    }
                }
            },
            _ => println!("We don't know what the power status of the TV is..."),
        }

        let sec = time::Duration::from_secs(1);
        thread::sleep(sec);
    }            
}

fn change_color(num_one: &u8, num_two: &u8, num_three: &u8) -> [RGB8; 150] {
    let color = RGB8::new(*num_one, *num_two, *num_three);
    let mut data = [RGB8::default(); NUM_LEDS];

    for i in 0..NUM_LEDS {
        data[i] = color;
    }

    data
}