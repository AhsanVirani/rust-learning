fn main() {
    let x: i32 = 10;

    // scope of var y
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }

    // this wont compile bc y is not in scope
    // println!("The value of x is {} and value of y is {}", x, y);
}