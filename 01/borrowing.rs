fn main() {
    let s1: String = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}


fn calculate_length(s: &String) -> usize {
    s.len()
}

// any number of immutable ref or single mutable not both