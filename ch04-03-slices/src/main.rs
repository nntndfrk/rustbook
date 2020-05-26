fn main() {
    let mut s = String::from("Hello world");

    let word = first_word(&s);

    println!("{}", word);

    // s.clear(); // cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("{}", word); // immutable borrow later used here

    // sliced array
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);
}

// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}