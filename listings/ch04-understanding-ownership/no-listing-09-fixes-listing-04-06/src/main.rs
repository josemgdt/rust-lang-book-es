fn main() {
    let mut s = String::from("hola");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", mundo");
}
