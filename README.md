# bleak
Change the aura of the room with your smart TV!

This project is licensed by the [Mozilla Public License v2](https://www.mozilla.org/en-US/MPL/2.0/).

## What is this??

This came from an idea in high school where one could change a channel and some LEDs could change color. This project is just that, only with Smart TV's (Roku TV's and Roku devices supported). One changes the application to Netflix, the LEDs turn red. Another changes the application to Hulu, the LEDs turn green.

## How does it work?

`bleak` sends out a API request to your Roku TV/Roku device you specify in a JSON file, which is read by `bleak`, and based on that response, the LEDs will change color. These requests happen at one second intervals.

TL;DR

Read the source code.

## Pre req's

* A Raspberry Pi (tested with a Raspberry Pi 4)
* A WS2818B LED strip
* Rust 1.40+ (this is the version of Rust `bleak` was written in) However, any version of Rust 2018 will most likely be fine

## Installation

### Hardware

Coming soon!

### Software

Download and install Rust. You can find Rust [here](https://www.rust-lang.org/) or just use `curl` to install Rust via rustup (recommended):

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Clone this repository:

`git clone https://github.com/wymillerlinux/bleak`

Asssuming one is a command line wizard, you have navigated to the root of the project.

## Compilation/Use

There's two ways of compiling this project. One way is to compile on Rasberry Pi itself, which is a bit slow. The other way is some cross-compilation hoodoo voodoo magic which I have not explored as of yet. Once I figure out to use cross-compliation effectively, I will update this readme.

Next, run `cargo` to compile it (I happened to compile this on the Raspberry Pi itself):

`cargo build --release && cp ./target/release/bleak ~/.cargo/bin && bleak`

or you can:

`cargo run`

to run this program as one is hacking away.

I wrote a systmed service file so starting and stopping would be like I'm starting and stopping any service on a Linux machine. I also didn't like `bleak` taking control of my terminal session :)

## Troubleshooting

Things that I've noticed:
* `bleak` will fail from time to time when grabbing responses. There's no error handling at this point.
* `bleak` like to change color to some random color(s) when being told to change color to, say, green or red.
* Roku TV's tend to be slow while `bleak` is running. Can't reproduce this problem, however...

## Smart TV support

In development:

* Roku devices and Roku TV's

Future possibilities:

* Samsung SmartThings TV
* Android TV
* Amazon's Fire TV

## Contribution

I'd love some contributors! Submit a PR and email me for some more information!
