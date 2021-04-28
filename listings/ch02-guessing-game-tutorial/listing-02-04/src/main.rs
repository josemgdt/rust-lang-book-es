// ANCHOR: here
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--
    // ANCHOR_END: here
    println!("Adivine el numero!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("El número secreto es: {}", secret_number);

    println!("Por favor, ingrese su suposición.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fallo al leer linea");
    // ANCHOR: here

    println!("Su suposición: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Demasiado pequeño!"),
        Ordering::Greater => println!("Demasiado grande!"),
        Ordering::Equal => println!("Acertó!"),
    }
}
// ANCHOR_END: here
