fn main() {
    let c: char = '*';

    let r1: &char = &c;

    let ref r2 = c;

    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("SUCCESS!")
}


// get memory address
fn get_addr(r: &char) -> String {
    format!("{:p}", r )
}