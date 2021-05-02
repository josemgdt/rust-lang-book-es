fn main() {
    // ANCHOR: here
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("uno"),
        3 => println!("tres"),
        5 => println!("cinco"),
        7 => println!("siete"),
        _ => (),
    }
    // ANCHOR_END: here
}
