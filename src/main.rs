mod bits;

use std::io;
use std::io::prelude::*;

const ERROR_RATE: f64 = 0.10;

fn main() {
    let mut buffer: Vec<u8> = Vec::new();
    match io::stdin().read_to_end(&mut buffer) {
        Err(why) => panic!("Cannot read: {}", why),
        Ok(_) => (),
    }

    // Randomly flip bits.
    for byte in buffer.iter_mut() {
        if fastrand::f64() <= ERROR_RATE {
            let index = fastrand::u8(0..=7);
            bits::flip(byte, index);
        }
    }

    // Write.
    match io::stdout().write(&buffer) {
        Err(why) => panic!("Cannot write: {}", why),
        Ok(_) => (),
    }
}
