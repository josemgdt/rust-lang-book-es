fn main() {
    // ANCHOR: here
    {
        let s = String::from("hello"); // s es valido desde este punto

        // hacer cosas con s
    }                                  // el alcance ha terminado, y s ya no
                                       // es válida
    // ANCHOR_END: here
}
