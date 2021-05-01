fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("El valor de y es: {}", y);
}
