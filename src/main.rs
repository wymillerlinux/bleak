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
mod request;
mod queue;

const NUM_LEDS: usize = 150;

fn main() {
    let mut queue: queue::Queue<RGB8> = queue::Queue::new();
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

                        if (queue.length() == NUM_LEDS) {
                            queue.dequeue();
                        }

                        queue.enqueue(data[i]);
                        ws.write(brightness(queue.queue.iter().cloned(), 32)).unwrap();
                    }
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
                            let color = RGB8::new(255, 0, 255);
                            let mut data = [RGB8::default(); NUM_LEDS];

                            for i in 0..NUM_LEDS {
                                data[i] = color;

                                if (queue.length() == NUM_LEDS) {
                                    queue.dequeue();
                                }

                                queue.enqueue(data[i]);
                                ws.write(brightness(queue.queue.iter().cloned(), 32)).unwrap();
                            }   
                        },
                        app::ActiveApp::Hulu => {
                            let color = RGB8::new(51, 255, 85);
                            let mut data = [RGB8::default(); NUM_LEDS];

                            for i in 0..NUM_LEDS {
                                data[i] = color;

                                if (queue.length() == NUM_LEDS) {
                                    queue.dequeue();
                                }

                                queue.enqueue(data[i]);
                                ws.write(brightness(queue.queue.iter().cloned(), 32)).unwrap();
                            }
                        },
                        app::ActiveApp::Netflix => {
                            let color = RGB8::new(255, 77, 77);
                            let mut data = [RGB8::default(); NUM_LEDS];

                            for i in 0..NUM_LEDS {
                                data[i] = color;

                                if (queue.length() == NUM_LEDS) {
                                    queue.dequeue();
                                }

                                queue.enqueue(data[i]);
                                ws.write(brightness(queue.queue.iter().cloned(), 32)).unwrap();
                            }
                        },
                        app::ActiveApp::AmazonPrime => {
                            let color = RGB8::new(99, 193, 255);
                            let mut data = [RGB8::default(); NUM_LEDS];

                            for i in 0..NUM_LEDS {
                                data[i] = color;

                                if (queue.length() == NUM_LEDS) {
                                    queue.dequeue();
                                }

                                queue.enqueue(data[i]);
                                ws.write(brightness(queue.queue.iter().cloned(), 32)).unwrap();
                            }
                        },
                        app::ActiveApp::Pandora => {
                            let color = RGB8::new(99, 123, 255);
                            let mut data = [RGB8::default(); NUM_LEDS];

                            for i in 0..NUM_LEDS {
                                data[i] = color;

                                if (queue.length() == NUM_LEDS) {
                                    queue.dequeue();
                                }

                                queue.enqueue(data[i]);
                                ws.write(brightness(queue.queue.iter().cloned(), 32)).unwrap();
                            }
                        },
                        app::ActiveApp::Spotify => {
                            let color = RGB8::new(51, 255, 85);
                            let mut data = [RGB8::default(); NUM_LEDS];

                            for i in 0..NUM_LEDS {
                                data[i] = color;

                                if (queue.length() == NUM_LEDS) {
                                    queue.dequeue();
                                }

                                queue.enqueue(data[i]);
                                ws.write(brightness(queue.queue.iter().cloned(), 32)).unwrap();
                            }
                        },
                        app::ActiveApp::Plex => {
                            let color = RGB8::new(255, 187, 51);
                            let mut data = [RGB8::default(); NUM_LEDS];

                            for i in 0..NUM_LEDS {
                                data[i] = color;

                                if (queue.length() == NUM_LEDS) {
                                    queue.dequeue();
                                }

                                queue.enqueue(data[i]);
                                ws.write(brightness(queue.queue.iter().cloned(), 32)).unwrap();
                            }
                        },
                        app::ActiveApp::Crunchyroll => {
                            let color = RGB8::new(255, 187, 51);
                            let mut data = [RGB8::default(); NUM_LEDS];

                            for i in 0..NUM_LEDS {
                                data[i] = color;

                                if (queue.length() == NUM_LEDS) {
                                    queue.dequeue();
                                }

                                queue.enqueue(data[i]);
                                ws.write(brightness(queue.queue.iter().cloned(), 32)).unwrap();
                            }
                        },
                        app::ActiveApp::Funimation => {
                            let color = RGB8::new(255, 0, 255);
                            let mut data = [RGB8::default(); NUM_LEDS];

                            for i in 0..NUM_LEDS {
                                data[i] = color;

                                if (queue.length() == NUM_LEDS) {
                                    queue.dequeue();
                                }

                                queue.enqueue(data[i]);
                                ws.write(brightness(queue.queue.iter().cloned(), 32)).unwrap();
                            }
                        },
                        app::ActiveApp::VRV => {
                            let color = RGB8::new(255, 187, 51);
                            let mut data = [RGB8::default(); NUM_LEDS];

                            for i in 0..NUM_LEDS {
                                data[i] = color;

                                if (queue.length() == NUM_LEDS) {
                                    queue.dequeue();
                                }

                                queue.enqueue(data[i]);
                                ws.write(brightness(queue.queue.iter().cloned(), 32)).unwrap();
                            }               
                        },
                        _ => println!("We don't know what app is running right now..."),
                    }
                }
            },
        }

        let sec = time::Duration::from_secs(1);
        thread::sleep(sec);
    }            
}
