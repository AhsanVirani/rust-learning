fn main() {
    define_x()
}

fn define_x() {
    let x: &str = "hello";
    println!("{}, world", x);

    // dont give warnings for unused as with let y = 1;
    let _y = 1;
}