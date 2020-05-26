fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    // move ->
    let s1 = s;

    // value borrowed here after move
    // println!("{}", s);
    println!("{}", s1);

    // deep copy!
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // simple types (stack-only) with Copy trait
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    println!("/ =========== /");

    // Ownership and Functions
    let s = String::from("Hello");
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5;
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward

    let s1 = give_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn give_ownership() -> String {
    let some_str = String::from("Hello");

    some_str
}

fn takes_and_gives_back(a_str: String) -> String {
    a_str
}
