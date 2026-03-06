use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let input: Vec<usize> = read();
    let mut t = input[0];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    while t > 0 {
        t -= 1;
        let input: Vec<String> = read();
        let r: i32 = input[0].parse().unwrap();
        let s = &input[1];

        for ch in s.chars() {
            for _ in 0..r {
                write!(writer, "{}", ch).unwrap();
            }
        }

        writeln!(writer).unwrap();
    }
}
