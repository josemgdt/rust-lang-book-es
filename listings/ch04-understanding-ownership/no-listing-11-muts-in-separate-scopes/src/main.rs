fn main() {
    // ANCHOR: here
    let mut s = String::from("hola");

    {
        let r1 = &mut s;
    } // r1 sale del alcance aqui, por lo que podemos hacer una referencia nueva sin problemas.

    let r2 = &mut s;
    // ANCHOR_END: here
}
