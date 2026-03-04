use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let input: Vec<usize> = read();
    let _n = input[0];
    let input: Vec<String> = read();
    let s = &input[0];
    let mut sum: u32 = 0;
    for c in s.chars() {
        sum += c.to_digit(10).unwrap();
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", sum).unwrap();
}
