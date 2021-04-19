fn main() {
    // ANCHOR: here
    {                      // s no es valido aqui, no se ha declarado aun
        let s = "hello";   // s es valido de aqui en adelante

        // cosas a hacer con s
    }                      // termina el alcance, y s deja de ser valida
    // ANCHOR_END: here
}
