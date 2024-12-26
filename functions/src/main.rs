fn main() {
    another_function(5);

    println!("Hello, world!");
    print_labeled_measurement(40, 'F');
    let five = five();
    println!("{five}")
}


fn another_function(x: i32){
    println!("The value of x is: {x}");
}


fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}


fn five() -> i32 { // in Rust, a funcition without semicolon is returned a value
    5
}