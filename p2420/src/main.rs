use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let n = input.next().unwrap();
    let m = input.next().unwrap();
    let mut popularity = n - m;
    if popularity < 0 {
        popularity = -popularity;
    }
    println!("{}", popularity);
}
