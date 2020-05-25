fn main() {
    // simple
    println!("Hello, world!");
    another_func(100, 42);

    // statement and expressions
    let x = 5;
    let y = {
        // statement don`t return a value
        let x = 3;

        // expression returned a value
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("The value of five is: {}", five());
    println!("The value of other five is: {}", other_five());
}

fn another_func(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn other_five() -> i32 {
    return 5;
}
