fn main() {
    let mut sum: i32 = 0;

    for i in -3..2 {
        sum += i;
    }

    println!("{}", sum);

    // include z 
    for c in 'a'..='z' {
        println!("{}", c);
    }
}