use std::io::{BufWriter, Write, stdin, stdout};
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
    let n = read::<usize>()[0];
    let idx_left = (n as f64 * 0.15).round() as usize;
    let idx_right = n - idx_left;
    let mut vec = Vec::new();
    for _ in 0..n {
        let i = read::<i32>()[0];
        vec.push(i);
    }
    vec.sort();
    let result = (vec[idx_left..idx_right].iter().sum::<i32>() as f64
        / (idx_right - idx_left) as f64)
        .round() as i32;
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", result).unwrap();
}
