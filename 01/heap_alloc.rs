fn main() {
    // allows i32 in heap
    let x: Box<i32> = Box::new(5);

    let mut y: Box<i32> = Box::new(1);

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!")
}