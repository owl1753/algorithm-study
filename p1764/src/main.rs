use std::collections::HashSet;
use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let input = read::<usize>();
    let n = input[0];
    let m = input[1];
    let mut s = HashSet::new();
    let mut vec = Vec::new();
    for _ in 0..n {
        s.insert(read::<String>()[0].clone());
    }
    for _ in 0..m {
        let str = &read::<String>()[0];
        if s.contains(str) {
            vec.push(str.clone());
        }
    }
    vec.sort();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", vec.len()).unwrap();
    for i in 0..vec.len() {
        writeln!(writer, "{}", vec[i]).unwrap();
    }
}
