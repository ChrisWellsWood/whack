extern crate whack;

use std::process;

fn main() {
    if let Err(e) = whack::run() {
        println!("Application error: {}", e);
        process::exit(1);
    };
}