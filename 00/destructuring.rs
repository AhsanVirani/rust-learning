fn main() {
    let (mut x, mut y) = (1, 2);
    println!("x: {} and y: {}", x, y);

    (x, ..) = (3, 4);
    println!("{}", x);

    [.., y] = [8, 5];
    println!("{}", y);
}