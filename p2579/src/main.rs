use std::cmp::{max};
use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let n = read::<usize>()[0];
    let mut vec = Vec::new();
    let mut d = vec![vec![-1; 301]; 3];
    vec.push(0);
    for _ in 0..n {
        vec.push(read::<i32>()[0]);
    }

    d[0][0] = 0;

    d[1][1] = vec[1];

    for i in 2..=n {
        for j in 0..=2 {
            if j != 2 && d[j][i - 1] != -1 {
                d[j + 1][i] = max(d[j + 1][i], d[j][i - 1] + vec[i]);
            }

            if d[j][i - 2] != -1 {
                d[1][i] = max(d[1][i], d[j][i - 2] + vec[i]);
            }
        }

    }

    let stdout  = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{}", max(d[1][n], d[2][n])).unwrap();
}