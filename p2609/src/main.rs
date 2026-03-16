use std::io::{stdin, stdout, BufWriter, Write};

fn gcd(n: i32, m: i32) -> i32 {
    if n % m == 0 {
        m
    } else {
        gcd(m, n % m)
    }
}

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let n = input.next().unwrap();
    let m = input.next().unwrap();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}\n{}", gcd(n, m), n / gcd(n, m) * m / gcd(n, m) * gcd(n, m)).unwrap();
}
