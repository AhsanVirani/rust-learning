use utf8_slice;

fn main() {
    let s = "The rocket goes to the moon!";

    let rocket = utf8_slice::slice(s, 4, 10);
    println!("{}", rocket);
}