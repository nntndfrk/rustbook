use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec!["blue".to_string(), "yellow".to_string()];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _>
        = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = "blue".to_string();

    let score = match scores.get(&team_name) {
        None => 0,
        Some(i) => **i
    };
    println!("The {} team has {} scores", team_name, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwriting a Value in hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Only Inserting a Value If the Key Has No Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
