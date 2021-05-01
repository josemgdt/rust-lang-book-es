fn main() {
    // ANCHOR: here
    let s = String::from("hola mundo");

    let hello = &s[0..5];
    let world = &s[4..10];
    println!("var hello es:{}--", hello);
    println!("var world es:{}--", world);
    // ANCHOR_END: here
}
