fn main() {
    // ANCHOR: here
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("Soy un 3");
    }
    // ANCHOR_END: here
}
