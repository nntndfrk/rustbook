fn main() {
    branches();
    println!("/==============/");
    let_operator();
    println!("/==============/");
    loop_loop();
    println!("/==============/");
    loop_return_value();
    println!("/==============/");
    while_loop();
    println!("/==============/");
    for_loop();
    println!("/==============/");
    for_loop_rev_range();
    println!("/==============/");
}

fn branches() {
    let number = 3;

    if number < 5 {
        println!("conditions was true");
    } else {
        println!("conditions was false");
    }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn let_operator() {
    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

fn loop_loop() {
    let mut c = 0;
    loop {
        if c == 2 {
            break;
        }
        println!("again!");
        c += 1;
    }
}

fn loop_return_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for el in a.iter() {
        println!("the value is: {}", el)
    }
}

fn for_loop_rev_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
