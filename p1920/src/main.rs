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
    let n: usize = read()[0];
    let mut vec = Vec::new();
    let input = read();
    for i in 0..n {
        let i: i32 = input[i];
        vec.push(i);
    }
    vec.sort();
    let mut m: usize = read()[0];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let input = read();
    for i in 0..m {
        let query = input[i];
        let result = vec.binary_search_by(|probe| probe.cmp(&query));
        match result {
            Ok(_idx) => writeln!(writer, "1").unwrap(),
            Err(_idx) => writeln!(writer, "0").unwrap(),
        }
    }
}
