fn main() {
    let mut s: String = String::from("Hello, ");

    let r1: &mut String = &mut s;
    r1.push_str("World");

    let r2: &mut String = &mut s;
    r2.push_str("!");

    println!("{}", r2);
}