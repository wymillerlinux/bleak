// external crates getting imported
extern crate reqwest;
extern crate select;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

// std lib imports
use std::{thread, time};
use std::error::Error;

// actual third party library being imported
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::gpio::Gpio;
use rppal::hal::{Delay, Timer};
use smart_leds::{colors, hsv::{Hsv, hsv2rgb}, RGB8, SmartLedsWrite, brightness};
use ws2812_spi::Ws2812;

// local files that need to be imported
mod app;
mod config;
mod generate;
mod led;

const GPIO_PIN: u8 = 19;
const NUM_LEDS: usize = 150;

fn main() {
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 3_000_000, Mode::Mode0).unwrap();
    let mut ws = Ws2812::new(spi);
    let delay = Delay::new();
    let mut configuration = config::init_config();

    loop {
        println!("{:?}", configuration);

        if false == configuration.is_change_app() {
            let app_text = configuration.get_app_status();
            configuration.change_active_app(&app_text);
            let activeapp = app::match_to_app(app_text);

            match activeapp {
                app::ActiveApp::Roku => {
                    let color = RGB8::new(255, 0, 255);
                    let mut data = [RGB8::default(); NUM_LEDS];

                    for i in 0..NUM_LEDS {
                        data[i] = color;
                    }
                    ws.write(brightness(data.iter().cloned(), 32)).unwrap();
                },
                app::ActiveApp::Hulu => {
                    let green = RGB8::new(51, 255, 85);
                    let mut data = [RGB8::default(); NUM_LEDS];

                    for i in 0..NUM_LEDS {
                        data[i] = green;
                    }
                    ws.write(brightness(data.iter().cloned(), 32)).unwrap();
                },
                app::ActiveApp::Netflix => {
                    let red = RGB8::new(255, 77, 77);
                    let mut data = [RGB8::default(); NUM_LEDS];

                    for i in 0..NUM_LEDS {
                        data[i] = red;
                    }
                    ws.write(brightness(data.iter().cloned(), 32)).unwrap();
                },
                app::ActiveApp::AmazonPrime => println!("The light are light blue!"),
                app::ActiveApp::Spotify => {
                    let green = RGB8::new(51, 255, 85);
                    let mut data = [RGB8::default(); NUM_LEDS];

                    for i in 0..NUM_LEDS {
                        data[i] = green;
                    }
                    ws.write(brightness(data.iter().cloned(), 32)).unwrap();
                },
                app::ActiveApp::Plex => {
                    let orange = RGB8::new(255, 187, 51);
                    let mut data = [RGB8::default(); NUM_LEDS];

                    for i in 0..NUM_LEDS {
                        data[i] = orange;
                    }
                    ws.write(brightness(data.iter().cloned(), 32)).unwrap();               
                },
                _ => println!("Oops!"),
            }
        }

        if false == configuration.is_change_power() {            
            let power_text = configuration.get_power_status();
            configuration.change_power(&power_text);
            let tvpower = app::match_to_power_status(power_text);

            match tvpower {
                app::TVPower::On => {
                    continue
                },
                app::TVPower::Off => {
                    let color = RGB8::new(0, 0, 0);
                    let mut data = [RGB8::default(); NUM_LEDS];

                    for i in 0..NUM_LEDS {
                        data[i] = color;
                    }

                    ws.write(brightness(data.iter().cloned(), 32)).unwrap();
                },
                _ => println!("We don't know what the power status of the TV is..."),
            }            
        }

        let sec = time::Duration::from_secs(3);
        thread::sleep(sec);
    }
}

fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}