use std::io;

fn main() {
    let mut nth = String::new();

    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line");

    let nth: usize = nth
        .trim()
        .parse()
        .expect("Index entered is not a number");

    let mut n1 = 0;
    let mut n2 = 1;

    println!("{:?}", n1);
    for _i in 0..nth {
        let aux = n1 + n2;
        n2 = n1;
        n1 = aux;
        println!("{:?}", aux);

    }
}
