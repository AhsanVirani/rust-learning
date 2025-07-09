fn main() {
    let s: &str = "hello, world";
    greetings(String::from(s)); // &str -> String

    println!("{}", s);
}

fn greetings(s: String) {
    println!("{}", s); 
}