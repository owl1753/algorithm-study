use std::io::{stdin, stdout, BufWriter, Write};

fn func(a: i64, b: i64) -> i64 {
    (a + b) * (a - b)
}

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let mut buffer = buffer.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let a = buffer.next().unwrap();
    let b = buffer.next().unwrap();

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", func(a, b)).unwrap();
}
