use std::env;
// Simple CLI frontend that prints the sum or arithmetic mean of numbers in a given file.
use mean::{mean_from_file, sum_from_file};

fn main() {
    let mut args = env::args().skip(1);
    let mut sum_mode = false;
    let first = match args.next() {
        Some(arg) if arg == "--sum" => {
            sum_mode = true;
            args.next()
        }
        Some(arg) => Some(arg),
        None => {
            eprintln!("Usage: mean [--sum] <file>");
            return;
        }
    };

    let path = match first {
        Some(p) => p,
        None => {
            eprintln!("Usage: mean [--sum] <file>");
            return;
        }
    };

    if sum_mode {
        match sum_from_file(&path) {
            Ok(value) => println!("{}", value),
            Err(msg) => eprintln!("{}", msg),
        }
    } else {
        match mean_from_file(&path) {
            Ok(value) => println!("{}", value),
            Err(msg) => eprintln!("{}", msg),
        }
    }
}
