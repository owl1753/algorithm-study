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
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    let n = input[0];
    let mut m = input[1];
    for i in 1..=n {
        let input = read::<String>();
        map1.insert(i, input[0].clone());
        map2.insert(input[0].clone(), i);
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    while m > 0 {
        m -= 1;
        let query = &read::<String>()[0];

        let result = query.parse::<usize>();

        match result {
            Ok(result) => {
                writeln!(writer, "{}", map1[&result]).unwrap();
            }
            Err(..) => {
                writeln!(writer, "{}", map2[query]).unwrap();
            }
        }
    }
}
