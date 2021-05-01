fn main() {
    let s = String::from("hola");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", mundo");
}
