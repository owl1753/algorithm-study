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
    let mut vec = Vec::new();
    for i in 0..n {
        let mut input: Vec<String> = read();
        let name = input.pop().unwrap();
        let age = input.pop().unwrap().parse::<i32>().unwrap();
        vec.push((i, age, name));
    }
    vec.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for el in vec {
        writeln!(writer, "{} {}", el.1, el.2).unwrap();
    }
}
