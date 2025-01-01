fn main() {
    // loop {
    //     println!("again!");
    // }

    let number = counter();
    println!("{:?}", number);

    loops_labels();
    println!("");
    while_loop();
    println!("");
    for_each();
    println!("");
    range_for();
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

fn while_loop(){
    let mut num = 3;

    while num != 0 {
        println!("{num}");

        num -= 1;
    }
}

fn for_each(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("{element}");
    }
}

fn range_for(){
    for i in (1..5).rev() {
        println!("{i}");
    }
}