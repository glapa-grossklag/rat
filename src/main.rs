mod bits;

use getopts::Options;
use std::io;
use std::io::prelude::*;

const DEFAULT_ERROR_RATE: f64 = 0.001;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut buffer: Vec<u8> = Vec::new();

    // Define arguments.
    let mut opts = Options::new();
    opts.optopt("e", "error", "The rate at which to flip bits.", "RATE");
    opts.optflag("h", "help", "Print this information.");

    // Match arguments.
    let matches = match opts.parse(&args[1..]) {
        Err(why) => panic!("{}", why.to_string()),
        Ok(m) => m,
    };

    // -h, --help
    if matches.opt_present("h") {
        let usage = format!("Usage:\n    {} [options]", args[0]);
        print!("{}", opts.usage(&usage));
        return;
    }

    // -e, --error
    let error_rate = match matches.opt_get_default("e", DEFAULT_ERROR_RATE) {
        Err(why) => panic!("Cannot read option: {}", why),
        Ok(x) => x,
    };

    // Read.
    match io::stdin().read_to_end(&mut buffer) {
        Err(why) => panic!("Cannot read: {}", why),
        Ok(_) => (),
    }

    // Randomly flip bits.
    for byte in buffer.iter_mut() {
        for bit in 0..8 {
            if fastrand::f64() <= error_rate {
                bits::flip(byte, bit);
            }
        }
    }

    // Write.
    match io::stdout().write(&buffer) {
        Err(why) => panic!("Cannot write: {}", why),
        Ok(_) => (),
    }

    return;
}
