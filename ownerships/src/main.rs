fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // concat text
    println!("{:?}", s); //OUTPUT: hello, world!
}
