use std::collections::HashMap;
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
    let mut t = read::<usize>()[0];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    while t > 0 {
        t -= 1;
        let n = read::<usize>()[0];
        let mut map: HashMap<String, i32> = HashMap::new();
        for _ in 0..n {
            let input = read::<String>();
            map.entry(input[1].clone())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut result = 1;
        for (_, count) in map.iter() {
            result *= count + 1;
        }
        result -= 1;
        writeln!(writer, "{}", result).unwrap();
    }
}
