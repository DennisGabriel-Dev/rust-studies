fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let b: [i32; 6] = [1, 2, 3, 4, 5, 6]; // array with six elements
    println!("{:?}", b);

    let c = [3; 5]; // create a array with length 5 and every elements equals 3
    println!("{:?}", c);

    println!("{:?}", b[1]);

    /*OUTPUT
    [1, 2, 3, 4, 5]
    [1, 2, 3, 4, 5, 6]
    [3, 3, 3, 3, 3]
    2
    */
}
