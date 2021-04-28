use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivine el numero!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Por favor, ingrese su suposición.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fallo al leer linea");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Su suposición: {}", guess);

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
