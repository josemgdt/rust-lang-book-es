fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Puede rect1 contener a rect2? {}", rect1.can_hold(&rect2));
    println!("Puede rect1 contener a rect3? {}", rect1.can_hold(&rect3));
}
