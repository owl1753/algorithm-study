use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let mut buffer = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut t = buffer.next().unwrap();

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    while t > 0 {
        t -= 1;
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let buffer: Vec<char> = buffer.trim().chars().collect();
        writeln!(writer, "{}{}", buffer[0], buffer[buffer.len() - 1]).unwrap();
    }
}
