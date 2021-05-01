fn main() {
    // ANCHOR: here
    {                      // s no es válido aqui, no se ha declarado aun
        let s = "hello";   // s es válido de aqui en adelante

        // cosas a hacer con s
    }                      // termina el alcance, y s deja de ser válida
    // ANCHOR_END: here
}
