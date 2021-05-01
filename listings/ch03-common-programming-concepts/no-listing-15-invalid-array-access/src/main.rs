use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Por favor, entre un índice de array.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Fallo al leer linea");

    let index: usize = index
        .trim()
        .parse()
        .expect("El índice no es un número");

    let element = a[index];

    println!(
        "El valor del elemento de índice {} es: {}",
        index, element
    );
}
