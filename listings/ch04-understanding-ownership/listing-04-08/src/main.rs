fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hola mundo");

    let word = first_word(&s); // word debe tomar el valor 5

    s.clear(); // esto vacia la String, haciendola igual a ""

    // word todavía tiene aqui el valor 5, pero no hay más cadena 
    // con la que podamos usar significativamente el valor 5. 
    // ¡ahora word es totalmente inválida!
}
// ANCHOR_END: here
