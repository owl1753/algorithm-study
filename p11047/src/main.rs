use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let input = read();
    let n = input[0];
    let mut k = input[1];
    let mut vec = Vec::new();
    let mut count = 0;
    for _ in 0..n {
        let input = read::<usize>();
        vec.push(input[0]);
    }
    for i in 0..n {
        loop {
            if k < vec[vec.len() - i - 1] {
                break;
            }

            k -= vec[vec.len() - i - 1];
            count += 1;
        }
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", count).unwrap();
}
