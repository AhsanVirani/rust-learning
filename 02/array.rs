fn main() {
    let _arr0: [i32; 3] = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    assert!(std::mem::size_of_val(&arr) == 12);
    
    println!("Success!");
}