fn main() {
    let s1 = String::from("hola");

    let (s2, len) = calculate_length(s1);

    println!("La longitud de '{}' es {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() retorna la longitud de una String

    (s, length)
}
