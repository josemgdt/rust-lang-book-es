fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hola mundo");

    let word = first_word(&s);

    s.clear(); // error!

    println!("la primera palabra es: {}", word);
}
// ANCHOR_END: here
