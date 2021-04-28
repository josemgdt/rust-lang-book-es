use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivine el numero!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("El número secreto es: {}", secret_number);

    loop {
        println!("Por favor, ingrese su suposición.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fallo al leer linea");

        let guess: u32 = guess.trim().parse().expect("Por favor, deme un numero!");

        println!("Su suposición: {}", guess);

        // ANCHOR: here
        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Demasiado pequeño!"),
            Ordering::Greater => println!("Demasiado grande!"),
            Ordering::Equal => {
                println!("Acertó!");
                break;
            }
        }
    }
}
// ANCHOR_END: here
