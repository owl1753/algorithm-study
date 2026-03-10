use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let input: Vec<usize> = read();
    let n = input[0];
    let input: Vec<i32> = read();
    let mut max = 0;
    let mut min = 0;
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for i in 0..n {
        if i == 0 {
            max = input[i];
            min = input[i];
        }

        if max < input[i] {
            max = input[i];
        }

        if min > input[i] {
            min = input[i];
        }
    }
    writeln!(writer, "{} {}", min, max).unwrap();
}
