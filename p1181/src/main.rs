use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer
        .split_ascii_whitespace()
        .flat_map(str::parse::<T>)
        .collect()
}

fn main() {
    let n: usize = read()[0];
    let mut vec: Vec<String> = Vec::new();
    for _ in 0..n {
        let mut input: Vec<String> = read();
        vec.push(input.remove(0));
    }
    vec.sort_by(|a, b| {
        if a.chars().count() == b.chars().count() {
            a.cmp(b)
        } else {
            a.chars().count().cmp(&b.chars().count())
        }
    });
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for i in 0..vec.len() {
        if i == 0 || i != 0 && vec[i - 1] != vec[i] {
            writeln!(writer, "{}", vec[i]).unwrap();
        }
    }
}
