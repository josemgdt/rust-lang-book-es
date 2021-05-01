fn main() {
    let s = String::from("hola");   // s entra en el alcance

    obtener_propiedad(s);           // el valor de s se mueve a la función ...
                                    // ... y por eso ya no es válido aquí

    let x = 5;                      // x entra en el alcance

    hacer_copia(x);                 // x se movería a la función,
                                    // pero i32 es Copy, así que es OK todavía
                                    // y usa x después

} // x sale del alcance, y luego s. Pero debido a que el valor de s se movió, nada
  // no ocurre nada especial.

fn obtener_propiedad(una_cadena: String) { // una_cadena entra en el alcance
    println!("{}", una_cadena);
} // una_cadena sale del alcance, se llama a `drop` y
  // se libera la memoria.

fn hacer_copia(un_entero: i32) { // un_entero entra en el alcance
    println!("{}", un_entero);
} //un_entero sale del alcance. No pasa nada especial.

