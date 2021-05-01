fn main() {
    // ANCHOR: here
    let mut s = String::from("hola");

    let r1 = &s; // sim problema
    let r2 = &s; // sin problema
    println!("{} and {}", r1, r2);
    // r1 y r2 no se usan despues de aqui

    let r3 = &mut s; // sin problema
    println!("{}", r3);
    // ANCHOR_END: here
}
