/*
this is the main rules:
Each value in Rust has an owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
 */
fn main(){                      // s is not valid here, it’s not yet declared
    let s = "42";   // s is valid from this point forward
        // do stuff with s
    println!("{s}");
    exemple_with_str();
    passing_the_value();
}                      // this scope is now over, and s is no longer validi
// ps: we have a important point to know and yet i dont know why... 
//      we cant call variableas before we call then but we can do this with functions and its very
//      normally and probably must organized because in this way the firt thing you see in the file
//      is the name of all the functions in the main functions
fn exemple_with_str(){
    //the first (s) dont exists here!!!
    let mut s = String::from("life");

    s.push_str(" = 42!"); // push_str() its like a append python function

    println!("{s}"); //this will print the appende function
}
fn passing_the_value(){
    let s1 = 5;
    let s2 = s1;
    let answer = s1 + s2;
    println!("{answer}");
}//this dont work in some types like strs... only for thas types who has knowed the time in compile
 //tyme you can check this for a better explication https://doc.rust-lang.org/reference/dynamically-sized-types.html
 
