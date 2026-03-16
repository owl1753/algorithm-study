use std::io::{stdin, stdout, BufWriter, Write};

fn read() -> Vec<i32> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<i32>).collect()
}

fn main() {
    let n = read()[0];
    let mut vec = Vec::new();
    for _ in 0..n {
        vec.push(read()[0]);
    }
    vec.sort();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for el in vec {
        writeln!(writer, "{}", el);
    }
}
