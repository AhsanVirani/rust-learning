use std::num::ParseIntError;

type Res<i32> = Result<i32, ParseIntError>;

fn multiply(first_numner_str: &str, second_number_str: &str) -> Res<i32> {
    first_numner_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print(result: Res<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("T", "2"));

    println!("Success");
}