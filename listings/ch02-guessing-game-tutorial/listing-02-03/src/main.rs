// ANCHOR: all
use std::io;
// ANCHOR: ch07-04
use rand::Rng;

fn main() {
    // ANCHOR_END: ch07-04
    println!("Adivine el numero!");

    // ANCHOR: ch07-04
    let secret_number = rand::thread_rng().gen_range(1..101);
    // ANCHOR_END: ch07-04

    println!("El número secreto es: {}", secret_number);

    println!("Por favor, ingrese su suposición.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fallo al leer linea");

    println!("Su suposición: {}", guess);
    // ANCHOR: ch07-04
}
// ANCHOR_END: ch07-04
// ANCHOR_END: all
