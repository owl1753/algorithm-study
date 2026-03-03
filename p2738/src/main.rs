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
    let input: Vec<usize> = read();
    let n = input[0];
    let m = input[1];
    let mut mat = vec![vec![0; m]; n];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for i in 0..n {
        let input: Vec<i32> = read();
        for j in 0..m {
            mat[i][j] = input[j];
        }
    }
    for i in 0..n {
        let input: Vec<i32> = read();
        for j in 0..m {
            write!(writer, "{} ", input[j] + mat[i][j]).unwrap();
        }
        writeln!(writer).unwrap();
    }
}
