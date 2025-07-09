fn main() {
    // immutable
    let x =  25;
    
    assert_eq!(x, 25);
    println!("{}", x);


    // mutable
    let mut y = 1;
    println!("{}", y);
    
    y = 5;
    println!("{}", y);
}