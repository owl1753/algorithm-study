use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let n = read::<usize>()[0];
    let mut vec = Vec::new();
    let mut count = vec![0; 8001];
    for _ in 0..n {
        let input = read::<i32>()[0];
        vec.push(input);
        count[(input + 4000) as usize] += 1;
    }
    let mut mode = 0;
    let mut max_count = 0;
    for i in 0..=8000 {
        if max_count < count[i] {
            mode = (i as i32) - 4000;
            max_count = count[i];
        }
    }
    for i in (mode + 4000) as usize + 1..=8000 {
        if max_count == count[i] {
            mode = (i as i32) - 4000;
            break;
        }
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    vec.sort();
    writeln!(writer, "{}", (vec.iter().sum::<i32>() as f64 / n as f64).round() as i32).unwrap();
    writeln!(writer, "{}", vec[n / 2]).unwrap();
    writeln!(writer, "{}", mode).unwrap();
    writeln!(writer, "{}", vec[n - 1] - vec[0]).unwrap();
}
