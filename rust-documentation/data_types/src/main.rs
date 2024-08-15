fn main(){
    /*
    Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusiv;
    e, where n is the number of bits that variant uses. 
    So an i8 can store numbers from -(27) to 27 - 1, 
    which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, 
    so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255  
    */

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
