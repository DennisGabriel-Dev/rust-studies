fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);

    let (x, y, z) = tup;

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);


    let five_hundred = tup.0;
    println!("{}", five_hundred);

    let six_point_four = tup.1;
    println!("{:?}", six_point_four);

    /* OUTPUT
    (500, 6.4, 1)
    500
    6.4
    1
    500
    6.4
     
    */
}
