fn main() {
    let mut s = String::from("hello"); // this format is a mutable string, rather using literals doesn´t  
    s.push_str(", world!"); // concat text
    println!("{:?}", s); //OUTPUT: hello, world!

    let s1 = String::from("hello");
    //let s2 = s1;
    //println!("{s1}"); //ERROR! because s1 isn't available in this scope anymore
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}"); // in this case s1 is available, the clone method makes a copy by s1 value 
}
