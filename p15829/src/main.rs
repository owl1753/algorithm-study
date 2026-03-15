use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

const R: i64 = 31;
const M: i64 = 1234567891;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn hashing(str: &String) -> i64 {
    let mut result = 0;
    for (i, ch) in str.chars().enumerate() {
        let mut tmp = 1;
        for _ in 0..i {
            tmp *= R;
            tmp %= M;
        }
        result += (((ch as i64 - 'a' as i64) + 1) * tmp) % M;
    }
    result % M
}

fn main() {
    let _: i32 = read()[0];
    let str = &read()[0];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", hashing(&str)).unwrap();
}
