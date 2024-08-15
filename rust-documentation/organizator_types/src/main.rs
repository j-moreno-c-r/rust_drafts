fn main() {
   tuples();
   array();
}

fn tuples() {
    let imperativetup:(i32, f64, u8) = (500, 6.4, 1);
    let simpletup = (210, 4.2, 3);
    let (arthur, bernard, carrol) = imperativetup;
    let(alice,bob, carlos) = simpletup;
    println!("This is a imperative tuple: {arthur}, {bernard}, {carrol}");
    println!("This is a acess in a simple declared tuple: {alice}, {bob}, {carlos}");
}
fn array() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];
    let atlas = months[0];
    println!("this is array acess {atlas}");
}
