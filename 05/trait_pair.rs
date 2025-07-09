struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl <T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {:?}", self.x);
        } else {
            println!("The largest number if y = {:?}", self.y);
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Unit(i32);

fn main() {

    let pair: Pair<Unit> = Pair::new(Unit(1), Unit(2));

    pair.cmp_display();
}