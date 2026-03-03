use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let s: Vec<char> = buffer.chars().collect();
    let input: Vec<usize> = read();
    let i = input[0];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    writeln!(writer, "{}", s[i - 1]).unwrap();
}
