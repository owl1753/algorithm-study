use std::io::{stdin, stdout, BufWriter, Write};

fn read() -> Vec<usize> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<usize>).collect()
}

fn main() {
    let mut t = read()[0];
    let mut d = vec![0i64; 101];
    d[1] = 1;
    d[2] = 1;
    d[3] = 1;
    d[4] = 2;
    d[5] = 2;
    for i in 6..=100 {
        d[i] = d[i - 1] + d[i - 5];
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    while t > 0 {
        t -= 1;
        let n = read()[0];
        writeln!(writer, "{}", d[n]).unwrap();
    }
}