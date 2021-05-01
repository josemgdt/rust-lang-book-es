fn main() {
    let s1 = dar_propiedad();                   // dar_propiedad mueve su valor de
                                                // retorno a s1

    let s2 = String::from("hola 1");            // s2 entra en el alcance

    println!("s2 por declaracion en main: {}", s2);

    let s3 = tomar_y_devolver(s2);              // s2 se mueve a
                                                // tomar_y_devolver , que también
                                                // mueve su valor de retorno a s3
    println!("s1 por retorno de dar propiedad: {}", s1);

    println!("s3 por retorno de tomar y devolver: {}", s3);

} // Aquí, s3 sale del alcance y se descarta. 
  // s2 sale del alcance pero se movió, por lo que no sucede nada. 
  // s1 sale del alcance y se descarta.

fn dar_propiedad() -> String {                  // dar_propiedad moverá su
                                                // valor de retorno a la llamada

    let alguna_cadena = String::from("hola 2"); // una_cadena entra en alcance

    println!("cadena por declaracion en alguna_cadena: {}", alguna_cadena);

    alguna_cadena                               // se devuelve alguna_cadena y
                                                // se mueve a la llamada
 
}

// tomar_y_devolver tomará una cadena y devolverá otra
fn tomar_y_devolver(una_cadena: String) -> String {     // una_cadena entra en
                                                        // el alcance
    println!("cadena por parametro s2 en tomar_y_devolver: {}", una_cadena);

    una_cadena  // se devuelve una_cadena y se mueve a la llamada
}

