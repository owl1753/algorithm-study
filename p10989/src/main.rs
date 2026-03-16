use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let n: usize = read()[0];
    let mut vec: Vec<usize> = vec![0; 10001];
    for _ in 0..n {
        let tmp: usize = read()[0];
        vec[tmp] += 1;
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for i in 1..=10000 {
        for _ in 0..vec[i] {
            writeln!(writer, "{}", i).unwrap();
        }
    }
}
