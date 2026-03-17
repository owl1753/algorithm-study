use std::io::{BufWriter, stdin, stdout, Write};
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
    let mut vec = Vec::new();
    for _ in 0..n {
        let input = read();
        let x: i32 = input[0];
        let y: i32 = input[1];
        vec.push((x, y));
    }
    vec.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for el in vec {
        writeln!(writer, "{} {}", el.0, el.1).unwrap();
    }
}
