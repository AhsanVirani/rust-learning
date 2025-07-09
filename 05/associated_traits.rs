trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming");
    }
}

struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn main() {
    let duck: Duck = Duck;
    duck.swim();

    let birds: [&dyn Bird; 2] = [&Duck, &Swan];

    for bird in birds {
        println!("{}", bird.quack());
    }
    // let bird: Box<dyn Bird> = hatch_a_bird(2);
    // assert_eq!(bird.quack(), "duck duck");

    // let bird: Box<dyn Bird> = hatch_a_bird(1);
    // assert_eq!(bird.quack(), "swan swan");

    // println!("Success!");
}

fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
    match species {
        1 => Box::new(Swan),
        2 => Box::new(Duck),
        _ => panic!(),
    }
}