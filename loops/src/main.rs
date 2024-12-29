fn main() {
    // loop {
    //     println!("again!");
    // }

    let number = counter();
    println!("{:?}", number);

    loops_labels();
}

fn counter() -> i32 {
    let mut counter = 0;

    let _result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    _result
}


fn loops_labels(){
    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");

        let mut remaing = 10;

        loop{
            println!("remaing = {remaing}");

            if remaing == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaing -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}