fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); // parse a string to number
    println!("{guess}"); // 42

    let x = 2.0; // as default x is a f64
    println!("{x}");

    let y: f32 = 3.0; // f32
    println!("{y}"); 

    let sum = 5 + 10;
    println!("{sum}");

    let difference = 95.5 - 4.3;
    println!("{difference}");

    let product = 4 * 30;
    println!("{product}");

    let quotient = 56.7 / 32.2;
    println!("{quotient}");

    let truncated = -5 / 3; // results in a integer number(the nearest)
    println!("{}", truncated); // -1

    let remainder = 43 % 5;
    println!("{}", remainder);

    let t = true;
    let f: bool = false;

    println!("{}, {}", t, f);

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}", c, z, heart_eyed_cat);

    /* OUTPUT

    42
    2
    3
    15
    91.2
    120
    1.7608695652173911
    -1
    3
    true, false
    z, Z, 😻
    */
}
