use std::io::{BufWriter, stdin, stdout, Write};

fn read() -> Vec<usize> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer
        .split_ascii_whitespace()
        .flat_map(str::parse::<usize>)
        .collect()
}

fn main() {
    let mut t = read()[0];
    let mut d = Vec::new();
    d.push((1, 0));
    d.push((0, 1));
    for i in 2..=40 {
        d.push((d[i - 1].0 + d[i - 2].0, d[i - 1].1 + d[i - 2].1));
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    while t > 0 {
        t -= 1;
        let n = read()[0];
        writeln!(writer, "{} {}", d[n].0, d[n].1).unwrap();
    }
}
