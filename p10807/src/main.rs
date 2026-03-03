use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let input1: Vec<usize> = read();
    let input2: Vec<i32> = read();
    let input3: Vec<i32> = read();
    let n = input1[0];
    let v = input3[0];
    let mut count = 0;
    for i in 0..n {
        if input2[i] == v {
            count += 1;
        }
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    write!(writer, "{}", count).unwrap();
}
