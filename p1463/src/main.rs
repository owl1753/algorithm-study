use std::cmp::min;
use std::io::{stdin, stdout, BufWriter, Write};

const MAX: usize = 1000000;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>).next().unwrap();
    let mut d = vec![i32::MAX; MAX + 1];
    d[1] = 0;
    for i in 2..=MAX {
        if i % 3 == 0 {
            d[i] = min(d[i], d[i / 3] + 1);
        }

        if i % 2 == 0 {
            d[i] = min(d[i], d[i / 2] + 1);
        }

        d[i] = min(d[i], d[i - 1] + 1);
    }

    let stdout  = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{}", d[n]).unwrap();
}
