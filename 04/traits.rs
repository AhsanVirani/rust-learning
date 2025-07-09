trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;   
}

struct Student {}
impl Hello for Student {
   fn say_something(&self) -> String {
    String::from("I'm a good student")
   }
}

struct Teacher {}
impl Hello for Teacher {
    
    fn say_hi(&self) -> String {
        String::from("I'm your new teacher")
    }
    
    fn say_something(&self) -> String {
        String::from("I'm a not a bad teacher")
    }
}

fn main() {
    let s: Student = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t: Teacher = Teacher {};
    assert_eq!(t.say_hi(), "I'm your new teacher");
    assert_eq!(t.say_something(), "I'm a not a bad teacher");    

    println!("Success");
}