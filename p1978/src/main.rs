use std::io::{stdin, stdout, BufWriter, Write};

fn read() -> Vec<i32> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>)
        .collect()
}

fn main() {
    let _ = read();
    let input = read();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let mut result = 0;
    for n in input {
        let mut count = 0;
        for i in 1..=n {
            if n % i == 0 {
                count += 1
            }
        }
        if count == 2 {
            result += 1;
        }
    }
    writeln!(writer, "{}", result).unwrap();
}
