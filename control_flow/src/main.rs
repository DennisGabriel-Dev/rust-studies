fn main() {
    let num = 7;
    if num < 5 {
        println!("condition was true!");
    } else {
        println!("condition was false!");
    }


    let num = 6;

    if num % 4 == 0 {
        println!("number is divisible by 4.");
    } else if num % 3 == 0 {
        println!("number is divisible by 3.");
    } else if num % 2 == 0 {
        println!("number is divisible by 2.");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let codition = true;
    let number = if codition { 5 } else { 6 }; // number receive 5.

    println!("The value of number is: {number}");
}
