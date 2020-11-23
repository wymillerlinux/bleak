![Bleak logo](docs/img/bleak_logo.png)

Change the aura of the room with your smart TV!

This project is licensed by the [Mozilla Public License v2](https://www.mozilla.org/en-US/MPL/2.0/). A copy of this license is in the project's root directory for your convenience.

The logo is licensed by the [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/).

## What is this??

This came from an idea in high school where one could change a channel and some LEDs could change color. This project is just that, only with Smart TV's (Roku TV's and Roku devices supported). One changes the application to Netflix, the LEDs turn red. Another changes the application to Hulu, the LEDs turn green.

## How does it work?

`bleak` sends out a API request to your Roku TV/Roku device you specify in a JSON file, which is read by `bleak`, and based on that response, the LEDs will change color. These requests happen at one second intervals.

TL;DR

Read the source code.

## Pre-requisites

* A Raspberry Pi (tested with a Raspberry Pi 4)
* A WS2818 LED light strip (you can order this from [Amazon](https://www.amazon.com/s?k=ws2818+led+strip))
* Rust 1.40+ (this is the version of Rust `bleak` was written in) However, any version of Rust 2018 will most likely be fine

## Installation

### Hardware

The hardware installation process isn't too tedious. However, it does take some knowledge of Raspberry Pi's GPIO and how it works. 

There should be three wires coming out the LED light strip. Typically, a red wire, a black wire, and another wire that could be any color besides red or black. 

Red can usually signify that this wire should be on a 5V GPIO pin. There are multiple so choose any that fits your fancy. 

Black can usually signify ground. So it should sit on a ground GPIO pin. Again, there are multiple. 

The wire I want to bring attention to is the other wire. This is the data wire. This wire holds all the data going to lights. This wire is placed on GPIO pin 19, an SPI wire. 

SPI is kind of neat but I'll spare you the details. If you would like to get more information on SPI, you can go read about it [here](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface).

Once that's all done, flash an operating system onto a SD card, plug the Raspberry Pi, and you're all set. Onward! To the software portion!

### Software

Download and install Rust. You can find Rust [here](https://www.rust-lang.org/) or just use `curl` to install Rust via rustup (recommended):

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

#### Option One

We all know `cargo` is pretty great, admit it. You can use `cargo` to install something directly from `git`, which is pretty cool. Run the following:

`cargo install --git https://scm.wyattjmiller.com/wyatt/bleak bleak`

`cargo` will download, compile, and install `bleak` for you, assuming that you have `cargo`'s bin directory in your `PATH`.

#### Option Two

Clone this repository:

`git clone https://github.com/wymillerlinux/bleak`

Asssuming one is a command line wizard, you have navigated to the root of the project.

## Compilation/Use

First, you have to manually create the JSON file for which `bleak` will read. 
There are five values:
- ipaddr
- port
- tv
- active_app
- power_status

The ipaddr and port keys will be pretty straight-forward. The tv, active_app, and power_status keys are arbitrary.

Here's the default JSON file you can use:

```json
{
    "ipaddr": "192.168.1.20",
    "port": 8060,
    "tv": "Roku",
    "active_app": "Roku",
    "power_status": "PowerOn"
}
```

Make sure it's saved under `config.json`.

**NOTE:** If you followed option one, you can skip the following, just run `bleak` or enable and run the systemd file, given you have copied and modififed the JSON file. If you followed option two, please continue.

Next, there's two ways of compiling this project. One way is to compile on Rasberry Pi itself, which is a bit slow. The other way is some cross-compilation hoodoo voodoo magic which I have not explored as of yet. Once I figure out to use cross-compliation effectively, I will update this readme.

Run `cargo` to compile it (I happened to compile this on the Raspberry Pi itself):

`cargo build --release && cp ./target/release/bleak ~/.cargo/bin && bleak`

or you can:

`cargo run`

to run this program as one is hacking away.

I wrote a systmed service file so starting and stopping would be like I'm starting and stopping any service on a Linux machine. I also didn't like `bleak` taking control of my terminal session :)

## Troubleshooting

Things that I've noticed:
* ~~`bleak` will crash from time to time when grabbing responses. There's no error handling at this point.~~
* `bleak` like to change color to some random color(s) when being told to change color to, say, green or red. I think that's just my light strip (or my breadboard I use for development) but I hav eno other light strips to test...
* ~~Roku TV's tend to be slow while `bleak` is running. Can't reproduce this problem, however~~ Doesn't seem to do this anymore

If you find any other issues with `bleak`, please send them my way in the form of a [new issue](https://scm.wyattjmiller.com/wyatt/bleak/issues/new).

## Smart TV support

In development:

* Roku devices and Roku TV's

Future possibilities:

* Samsung SmartThings TV
* Android TV
* Amazon's Fire TV

## Contribution

I'd love some contributions! Submit an issue or a PR and email me for some more information!
