use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let mut buffer = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = buffer.next().unwrap();

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for i in 1..=n {
        for _ in 0..n - i {
            write!(writer,  " ").unwrap();
        }
        for _ in 0..i {
            write!(writer, "*").unwrap();
        }
        writeln!(writer).unwrap();
    }
}
