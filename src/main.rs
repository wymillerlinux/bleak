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
use smart_leds::{RGB8, SmartLedsWrite, brightness};
use ws2812_spi::Ws2812;
use embedded_hal;

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
    let configuration = config::init_config();
    println!("{:?}", configuration);


    // let gil = Python::acquire_gil();
    // let py = gil.python();
    // let m = PyModule::import(py, "led.py").unwrap();

    loop {
        let app_text = configuration.get_tv_status();
        let power_text = configuration.get_power_status();

        let activeapp = app::match_to_app(app_text);
        let tvpower = app::match_to_power_status(power_text);

        match activeapp {
            app::ActiveApp::Roku => println!("The lights are light purple!"),
            app::ActiveApp::Netflix => {
                let green = RGB8::new(255, 0, 0);
                let mut data = [RGB8::default(); NUM_LEDS];

                for j in 0..(256 * 5) {
                    for i in 0..NUM_LEDS {
                        data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
                    }
                    ws.write(brightness(data.iter().cloned(), 32)).unwrap();
                }
            },
            app::ActiveApp::Hulu => println!("The lights are green!"),
            app::ActiveApp::AmazonPrime => println!("The light are light blue!"),
            app::ActiveApp::Spotify => println!("The lights are light green!"),
            _ => println!("Oops!"),
        }

        match tvpower {
            app::TVPower::On => println!("TV is on!"),
            app::TVPower::Off => println!("TV is off"),
            _ => println!("We don't know what the power status of the TV is..."),
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