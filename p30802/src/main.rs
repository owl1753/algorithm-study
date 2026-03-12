use std::io::{BufWriter, Write, stdin, stdout};

fn read() -> Vec<i32> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>)
        .collect()
}

fn main() {
    let n = read()[0];
    let apps = read();
    let input = read();
    let t = input[0];
    let p = input[1];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut result = 0;
    for app in apps {
        result += app / t;
        result += if app % t != 0 { 1 } else { 0 }
    }

    writeln!(writer, "{}", result).unwrap();
    writeln!(writer, "{} {}", n / p, n % p).unwrap();
}
