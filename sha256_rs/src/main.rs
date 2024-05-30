use sha2::{Sha256, Digest};
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter a string: ");
    io::stdout().flush().unwrap(); // Flush the output
    io::stdin().read_line(&mut input).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    println!("SHA-256 hash: {:x}", result);}