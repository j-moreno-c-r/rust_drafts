fn main() {
    let g=5.0 ; // publicly known
    let x=7.0 ; // only Alice knows this 
    let y=3.0 ; //only Bob knows this
    let aliceSends = f32::powf(g,x);
    let bobSends = f32::powf(g, y);
    let aliceComputes = f32::powf(bobSends,x);
    let bobComputes = f32::powf(aliceSends, y);
    println!("Alice sends: 💌 {aliceSends}");
    println!("Bob sends: 💌 {bobSends} ");
    println!("Bob computes: ✏️ {bobComputes}");
    println!("Alice computes:✏️ {aliceComputes}");
}
