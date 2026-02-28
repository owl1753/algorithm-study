use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let year = input.next().unwrap();

    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        println!("1");
    } else {
        println!("0");
    }
}
