fn main() {
    // ANCHOR: here
    let s1 = String::from("Hola, ");
    let s2 = String::from("mundo!");
    let s3 = s1 + &s2; // s1 ha sido movida aqui y no se puede volver a usar
                       // ANCHOR_END: here
}
