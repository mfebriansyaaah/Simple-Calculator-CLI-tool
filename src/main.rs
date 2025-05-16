fn plus_ten(a: i32) -> i32 {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);
    a
}

fn main() {
    println!("Angka: {}", plus_ten(10));
}