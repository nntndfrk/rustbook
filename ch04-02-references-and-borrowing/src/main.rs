fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s);

    let r1 = &mut s;
    // let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time
    println!("{}", r1);

    // many immutable or one mutable
    let mut s = String::from("hello");
    let r1 = &s; // OK
    let r2 = &s; // OK
    // let r3 = &mut s; // cannot borrow `s` as mutable because it is also borrowed as immutable
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_str: &mut String) {
    some_str.push_str(", world");
}