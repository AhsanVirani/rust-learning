trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", self) }
}

// Implement with generics - only on args that implements Foo trait
fn static_dispath<T: Foo>(a: T) {
    a.method();
}

fn dynamic_dispatch(a: &dyn Foo) {
    a.method();
}

fn main() {
    let x: u8 = 5u8;
    let y: String = "Hello".to_string();

    static_dispath(x);
    dynamic_dispatch(&y);

    println!("Success");
}