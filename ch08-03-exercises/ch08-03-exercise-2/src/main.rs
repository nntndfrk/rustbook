fn main() {
    println!(
        "{}",
        translate_to_pig_latin("first")
    );
    println!(
        "{}",
        translate_to_pig_latin("apple")
    );
    println!(
        "{}",
        translate_to_pig_latin("Three")
    );
    println!(
        "{}",
        translate_to_pig_latin("question")
    );
}

fn translate_to_pig_latin(text: &str) -> String {
    const VOWELS: &str = "aeiouyAEIOUY";
    const SEPARATOR: &str = "-";

    let mut translated = String::new();
    let mut tail = String::new();

    let mut translate_mod = true;
    let mut is_vowel_first = false;

    for c in text.chars() {
        if translate_mod {
            if !VOWELS.contains(c) {
                tail.push(c);
            } else {
                if tail.len() == 0 {
                    is_vowel_first = true;
                }
                translated.push(c);
                translate_mod = false;
            }
        } else {
            translated.push(c);
        }
    }

    if is_vowel_first {
        tail.push_str("hay");
    } else {
        tail.push_str("ay")
    }

    translated + SEPARATOR + &tail
}