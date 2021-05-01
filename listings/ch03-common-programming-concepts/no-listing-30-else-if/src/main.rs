fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("número divisible entre 4");
    } else if number % 3 == 0 {
        println!("número divisible entre 3");
    } else if number % 2 == 0 {
        println!("número divisible entre 2");
    } else {
        println!("número no divisible entre 4, 3, o 2");
    }
}
