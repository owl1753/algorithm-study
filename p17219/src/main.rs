use std::collections::HashMap;
use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let input = read::<usize>();
    let mut map = HashMap::new();
    let n = input[0];
    let m = input[1];
    for _ in 0..n {
        let input = read::<String>();
        map.insert(input[0].clone(), input[1].clone());
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for _ in 0..m {
        let query = &read::<String>()[0];
        writeln!(writer, "{}", map[query]).unwrap();
    }
}
