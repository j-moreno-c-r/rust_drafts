use std::io;
fn main() {
    println!("type a number:"); let mut index = String::new();   
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    
    println!("your number is {index}");
    if index % 3 == 0 {
        println!("fizz");
    }if index % 5 == 0 {
        println!("buzz")
    }

}   
