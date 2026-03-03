use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap();
    let x = input.next().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);

    for i in 0..n {
        let el = input.next().unwrap();
        if el < x {
            write!(writer, "{} ", el).unwrap();
        }
    }
}
