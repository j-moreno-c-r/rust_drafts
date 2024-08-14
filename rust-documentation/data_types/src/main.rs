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
    let imperativechar: &str = "noob"; //str are diferent of char double "" and multiple caractere
    
    println!("A &str:{simplechar} ");
    println!("A imperative char:{imperativechar}");
    println!("A num:{simplenum} ");
    println!("A floats:{simplefloat} ");
    println!("A Bolean:{simpleboolean} ");
}
