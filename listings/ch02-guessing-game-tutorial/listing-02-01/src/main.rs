// ANCHOR: all
// ANCHOR: io
use std::io;
// ANCHOR_END: io

// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: print
    println!("Adivine el numero!");

    println!("Por favor, ingrese su suposición.");
    // ANCHOR_END: print

    // ANCHOR: string
    let mut guess = String::new();
    // ANCHOR_END: string

    // ANCHOR: read
    io::stdin()
        .read_line(&mut guess)
        // ANCHOR_END: read
        // ANCHOR: expect
        .expect("Fallo al leer linea");
    // ANCHOR_END: expect

    // ANCHOR: print_guess
    println!("Su suposición: {}", guess);
    // ANCHOR_END: print_guess
}
// ANCHOR: all
