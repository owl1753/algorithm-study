use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>).next().unwrap();
    let mut i = 1;
    let mut tmp = 1;
    loop {
        if n <= tmp {
            break;
        }
        tmp += i * 6;
        i += 1;
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", i).unwrap();
}
