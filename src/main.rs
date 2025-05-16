fn main() {
    let say_hello = "Hello";
    let mut name = String::from("World");
    pushing(&mut name);
    println!("{} {}", say_hello, name);
}

fn pushing(a: &mut String) {
    a.push_str("!!");
}