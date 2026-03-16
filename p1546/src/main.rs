use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let n = read()[0];
    let grades: Vec<i32> = read();
    let mut sum: f64 = 0.0;
    let mut max = None;
    for i in 0..n {
        max = match max {
            None => Some(grades[i]),
            Some(max) => if max < grades[i] { Some(grades[i])} else { Some(max) }
        }
    }
    for i in 0..n {
        sum += grades[i] as f64 / max.unwrap() as f64 * 100.0;
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", sum / n as f64).unwrap();
}
