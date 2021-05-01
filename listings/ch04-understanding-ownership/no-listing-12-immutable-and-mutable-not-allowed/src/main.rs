fn main() {
    // ANCHOR: here
    let mut s = String::from("hola");

    let r1 = &s; // sin problema
    let r2 = &s; // sin problema
    let r3 = &mut s; // GRAN PROBLEMA

    println!("{}, {}, and {}", r1, r2, r3);
    // ANCHOR_END: here
}
