fn main() {
    let s = String::from("Hello");

    takes_ownership(s);

    // this wont run because s is not freed 
    // println!("{}", s);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}