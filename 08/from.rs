#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self {
            value
        }
    }
}

fn main() {
    let num: Number = Number::from(40);
    assert_eq!(num.value, 40);

    let num: Number = 40.into();
    assert_eq!(num.value, 40);

    println!("Success");
}