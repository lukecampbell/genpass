//! genpass Quick password generation (now in Rust)
//!
//! TODO Long Description
//!
//! # Examples
//!
//! TODO Example
use rand::seq::IndexedRandom;

use clap::Parser;
const ALNUM: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const SPECIALS: &[u8] = b"~!@#$%^&*()_+`-=[]{};':\",./<>?\\|";
const SEMI_SAFE: &[u8] = b"_+-=?";

/// Program Arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'n', long, default_value_t = 32)]
    length: u16,

    /// Generate password containing random alphanumeric and special characters
    #[arg(short, long, default_value_t = false)]
    secure: bool,

    /// Generate password containing special characters that are less likely to cause issue
    #[arg(short = 'u', long, default_value_t = false)]
    semi_secure: bool,
}

/// Main entry point
fn main() {
    // Parse arguments
    let args = Args::parse();
    // Initialize the RNG
    let mut rng = rand::rng();
    // Create our options to choose from
    let options: Vec<u8> = {
        let mut tmp_opts: Vec<u8> = ALNUM.to_vec();
        if args.secure {
            SPECIALS.iter().for_each(|c| tmp_opts.push(*c));
        } else if args.semi_secure {
            SEMI_SAFE.iter().for_each(|c| tmp_opts.push(*c));
        }
        tmp_opts
    };
    // Choose from our options
    let mut buf: Vec<u8> = Vec::new();
    for _ in 0..args.length {
        let c = options
            .choose(&mut rng)
            .expect("Failed to generate a random number.");
        buf.push(*c);
    }
    println!("{}", std::str::from_utf8(&buf).unwrap());
}
