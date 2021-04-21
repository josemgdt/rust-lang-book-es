fn main() {
    let s = String::from("hello");  // s entra en el alcance

    takes_ownership(s);             // el valor de s se mueve a la función ...
                                    // ... y por eso ya no es válido aquí

    let x = 5;                      // x entra en el alcance

    makes_copy(x);                  // x se movería a la función,
                                    // pero i32 es Copy, así que es OK todavía
                                    // y usa x después

} // x sale del alcance, y luego s. Pero debido a que el valor de s se movió, nada
  // no ocurre nada especial.

fn takes_ownership(some_string: String) { // some_string entra en el alcance
    println!("{}", some_string);
} // some_string sale del alcance, se llama a `drop` y
  // se libera la memoria.

fn makes_copy(some_integer: i32) { // some_integer entra en el alcance
    println!("{}", some_integer);
} //some_integer sale del alcance. No pasa nada especial.

