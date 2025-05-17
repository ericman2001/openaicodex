use std::env;
// Simple CLI frontend that prints the mean of numbers in a given file.
use mean::mean_from_file;

fn main() {
    // Expect a single command line argument specifying the file to read.
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("Usage: mean <file>");
            return;
        }
    };

    // Compute the mean and print the result or any error encountered.
    match mean_from_file(&path) {
        Ok(value) => println!("{}", value),
        Err(msg) => eprintln!("{}", msg),
    }
}
