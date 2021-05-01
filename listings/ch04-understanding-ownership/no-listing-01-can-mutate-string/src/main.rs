fn main() {
    // ANCHOR: here
    let mut s = String::from("hola");

    s.push_str(", mundo!"); // push_str() añade un literal a una String

    println!("{}", s); // Esto debe imprimir `hola, mundo!`
                       // ANCHOR_END: here
}
