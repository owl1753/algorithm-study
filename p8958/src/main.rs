use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T>{
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let mut t: usize = read()[0];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    while t > 0 {
        t -= 1;
        let str: &String = &read()[0];
        let mut count = 0;
        let mut score = 0;
        for ch in str.chars() {
            if ch == 'O' {
                count += 1;
            } else {
                count = 0;
            }
            score += count;
        }
        writeln!(writer, "{}", score).unwrap();
    }
}
