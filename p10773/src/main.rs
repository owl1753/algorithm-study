use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer
        .split_ascii_whitespace()
        .flat_map(str::parse::<T>)
        .collect()
}

fn main() {
    let k: usize = read()[0];
    let mut vec = Vec::new();

    for _ in 0..k {
        let i: i32 = read()[0];
        match i {
            0 => {
                vec.pop();
            }
            _ => {
                vec.push(i);
            }
        }
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let sum: i32 = vec.iter().sum();

    writeln!(writer, "{}", sum).unwrap();

}
