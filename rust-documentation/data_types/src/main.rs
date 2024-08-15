fn main(){
    //its important to listen the integers type
    //they use a more lower level idea
    //the idea is you set the syze in bytes like C 
    // i'size' for sigened content
    // or u'size' for unsigned content
    let simplechar = "rustnoob";
    let simplenum = 42;
    let simplefloat = 0.42;//default type is f64
    let simpleboolean = true;
    //but you can declare in a imperative way!
    let imperativestr: &str = "noob"; //str are diferent of char double "" and multiple caractere
    //diferent of:
    let imperativechar: char = 'b'; //of ....you know....
    let imperativenum: i16 = 21;// in num imperative declaration set the size and type
    let imperativeboolean: bool = false;
    println!("A &str:{simplechar}");
    println!("A num:{simplenum}");
    println!("A floats:{simplefloat}");
    println!("A Bolean:{simpleboolean}");
    println!("A imperative str:{imperativestr}");
    println!("A imperative char:{imperativechar}");
    println!("A imperative num:{imperativenum}");
    println!("A imperative boolean:{imperativeboolean}");
}
