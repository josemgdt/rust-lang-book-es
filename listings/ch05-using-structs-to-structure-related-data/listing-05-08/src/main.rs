// ANCHOR: all
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "el área del rectángulo es {} pixels al cuadrado.",
        area(width1, height1)
    );
}

// ANCHOR: here
fn area(width: u32, height: u32) -> u32 {
    // ANCHOR_END: here
    width * height
}
// ANCHOR_END: all
