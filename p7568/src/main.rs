use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let n: usize = read()[0];
    let mut vec = Vec::new();
    let mut result = vec![1; n];
    for _ in 0..n {
        let input: Vec<i32> = read();
        vec.push((input[0], input[1]));
    }
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if vec[i].0 < vec[j].0 && vec[i].1 < vec[j].1 {
                result[i] += 1;
            }
        }
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for el in result {
        write!(writer, "{} ", el).unwrap();
    }

}
