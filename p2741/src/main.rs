use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap();

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);

    for i in 1..n + 1{
        writeln!(writer, "{i}",).unwrap();
    }
}
