fn main() {
    let s1 = String::from("hola");

    let len = calculate_length(&s1);

    println!("La longitud de '{}' es {}.", s1, len);
}

// ANCHOR: here
fn calculate_length(s: &String) -> usize { // s es una referencia a una String
    s.len()
} // s sale del alcance. Pero como no tiene la propiedad de a lo que
  // se refiere, no sucede nada.
// ANCHOR_END: here
