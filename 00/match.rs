fn main() {
    println!("Success");
    get_options(1);
}

fn get_options(tp: u8) -> Option<i32> {
    match tp {
        1 => Some(100),
        _ => None,
    }
}