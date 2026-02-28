use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let x = input.next().unwrap();
    let y = input.next().unwrap();
    let quadrant: i32;

    if x > 0 && y > 0 {
        quadrant = 1;
    } else if x < 0 && y > 0 {
        quadrant = 2;
    } else if x < 0 && y < 0 {
        quadrant = 3;
    } else {
        quadrant = 4;
    }

    println!("{}", quadrant);
}
