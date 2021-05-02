fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // ANCHOR: here
    impl Message {
        fn call(&self) {
            // el cuerpo del método se debe definir aqui
        }
    }

    let m = Message::Write(String::from("hola"));
    m.call();
    // ANCHOR_END: here
}
