use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let _ = read::<usize>()[0];
    let mut input = read::<i32>();
    let mut sum = 0;
    input.sort();
    for i in 0..input.len() {
        let mut tmp = 0;
        for j in 0..=i {
            tmp += input[j];
        }
        sum += tmp;
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", sum).unwrap();
}
