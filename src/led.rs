extern crate rppal;

use rppal::spi::{Bus, Mode, Spi, SlaveSelect};

/// Struct for storing grb values
pub struct Led {
    buffer: Vec<u8>,
    spi: Spi,
    num_leds: u32,
}

/// Struct for storing rgb values
pub struct ColorRGB (pub u8, pub u8, pub u8);

impl Led {
    /// create a Led instance
    pub fn new(num_leds: u32) -> Led {
        // initlaize all the things
        // like the buffer, the bus, the mode that we want the SPI in,
        // the slave and the clock speed
        let buffer = Vec::new();
        let bus = Bus::Spi0;
        let mode = Mode::Mode0;
        let slave = SlaveSelect::Ss0;
        let clock = 3 * 1000 * 1000;
        
        // actually create a new instance of this thing and return it
        Led {
            buffer,
            spi: Spi::new(bus, slave, clock, mode)
                .unwrap(),
            num_leds
        }
    }

    // Write out to the leds to the controller
    fn write(&mut self) {
        // collects all the bytes to write
        let output = self.buffer
            .drain(..)
            .flat_map(|val| byte_to_spi_bytes(val).to_vec())
            .collect::<Vec<u8>>();

        self.spi.write(&output).unwrap();
    }

    /// Set the LEDs so that they could be written to the conntroller
    /// This method is written for converting RGB to GRB
    pub fn set_leds(&mut self, hex_codes: &[ColorRGB]) {
        hex_codes
            .iter()
            .for_each(|hex_code| {
                // swapping here from RGB to the GRB expected
                self.buffer.extend_from_slice(&[hex_code.1, hex_code.0, hex_code.2]);
            });
        self.write();
    }

    /// Turns all LEDs off and clears buffer
    /// Useful for when the TV is off
    pub fn clear_all_leds(&mut self) {
        self.buffer.clear();
        let mut clear_codes = vec![0; (self.num_leds * 3) as usize];
        self.buffer.append(&mut clear_codes);
        self.write();
    }

}

/// Function to convert bytes to SPI bytes
pub fn byte_to_spi_bytes(input: u8) -> [u8; 3] {
    // first convert the u8 to 24 bits
    let mut bool_array = [false; 24];
    for bit_index in 0..8 {
        let bit = input & (1 << bit_index) != 0;
        let out_index = bit_index * 3;

        // first bit is always 0
        // this could be omitted because the array is initialized to false
        // last bit is always 1
        bool_array[out_index] = false;
        bool_array[out_index + 1] = bit;
        bool_array[out_index + 2] = true;
    }

    // then convert the 24 bits to three u8
    [
        bool_slice_to_u8(&bool_array[0..8]),
        bool_slice_to_u8(&bool_array[8..16]),
        bool_slice_to_u8(&bool_array[16..24]),
    ]
}

/// Function that checks if there are eight booleans 
pub fn bool_slice_to_u8(input: &[bool]) -> u8 {
    if input.len() != 8 { panic!("bool to u8 conversion requires exactly 8 booleans") }

    let mut out = 0b0000_0000u8;

    for (carry_bit, flag) in input.iter().enumerate() {
        if *flag { out += 0b0000_0001u8 << carry_bit }
    }

    out
}