// ANCHOR: here
struct QuitMessage; // struct unitaria
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // struct tupla
struct ChangeColorMessage(i32, i32, i32); // struct tupla
                                          // ANCHOR_END: here

fn main() {}
