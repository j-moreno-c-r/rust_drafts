
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter a string: ");
    io::stdout().flush().unwrap(); // Flush the output
    io::stdin().read_line(&mut input).unwrap();
}

    
