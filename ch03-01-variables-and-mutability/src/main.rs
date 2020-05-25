const MAX_POINTS: u32 = 100_000;

fn main() {
    // mutability vars
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;

    // shadowing
    let x = x + 1;
    let x = x * 2;

    // shadowing override types
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of x is: {}", x);
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}
