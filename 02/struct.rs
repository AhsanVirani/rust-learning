struct Point(i32, i32, i32);

fn main() {
    let v: Point = Point(0, 127, 255);
    check_color(v);

    println!("Success");
}


fn check_color(p: Point) {
    let Point(x, y, z) = p;

    assert_eq!(x, 0);
    assert_eq!(y, 127);
    assert_eq!(z, 255);
}