use std::collections::HashMap;
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
    let _: usize = read()[0];
    let input: Vec<i32> = read();
    let mut hashmap = HashMap::new();
    for el in input {
        hashmap
            .entry(el)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let _: usize = read()[0];
    let input: Vec<i32> = read();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for el in input {
        let count = hashmap.get(&el);
        match count {
            Some(count) => write!(writer, "{} ", *count).unwrap(),
            None => write!(writer, "0 ").unwrap(),
        }
    }
}
