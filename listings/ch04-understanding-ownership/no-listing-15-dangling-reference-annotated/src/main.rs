fn main() {
    let reference_to_nothing = dangle();
}

// ANCHOR: here
fn dangle() -> &String { // dangle retorna una referencia a una String

    let s = String::from("hello"); // s es una nueva String

    &s // se retorna una referencia a la String, s
} // Aqui, s sale de alcance y se descarta. Su memoria se libera.
  // Peligro!
// ANCHOR_END: here
