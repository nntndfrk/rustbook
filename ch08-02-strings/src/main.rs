fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // UTF-8
    let hello = String::from("Здравствуйте");

    let mut s = String::from("foo");
    s.push_str(" bar");
    println!("{}", s);

    s.push(' ');
    s.push('b');
    s.push('a');
    s.push('z');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");

    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }

    for c in "Здравствуйте".bytes() {
        println!("{}", c);
    }
}
