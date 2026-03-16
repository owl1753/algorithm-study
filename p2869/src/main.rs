use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let v = input.next().unwrap();

    let day = if (v - a) % (a - b) != 0 { (v - a) / (a - b) + 2 } else { (v - a) / (a - b) + 1 };

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{}", day).unwrap();
}
