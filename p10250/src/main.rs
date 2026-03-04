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
    let input: Vec<usize> = read();
    let mut t = input[0];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    while t > 0 {
        t -= 1;
        let input: Vec<i32> = read();
        let h = input[0];
        let w = input[1];
        let n = input[2];
        writeln!(
            writer,
            "{}{:02}",
            if n % h == 0 { h } else { n % h },
            (n - 1) / h + 1,
        )
        .unwrap();
    }
}
