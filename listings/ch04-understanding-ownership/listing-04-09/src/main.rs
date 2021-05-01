// ANCHOR: here
fn first_word(s: &str) -> &str {
    // ANCHOR_END: here
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: usage
fn main() {
    let my_string = String::from("hola mundo");

    // first_word trabajando en slices de `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hola mundo";

    // first_word trabajando en slice de string literal
    let word = first_word(&my_string_literal[..]);

    // Ya que los string literal *son* string slices tambien,
    // esto trabaja sin la sintaxis de slices!
    let word = first_word(my_string_literal);
}
// ANCHOR_END: usage
