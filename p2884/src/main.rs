use std::io::{BufWriter, Write, stdin, stdout};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let mut buffer = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut h = buffer.next().unwrap();
    let mut m = buffer.next().unwrap();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    m -= 45;

    if m < 0 {
        m += 60;
        h -= 1;
    }

    if h < 0 {
        h += 24;
    }

    writeln!(writer, "{} {}", h, m).unwrap();
}
