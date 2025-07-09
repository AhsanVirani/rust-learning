enum Direction {
    East,
    West,
    South,
    North,
}

fn main() {
    let dire: Direction = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}