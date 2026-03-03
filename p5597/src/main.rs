use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let mut vec: Vec<bool> = vec![false; 30];

    for i in 0..28 {
        let input: Vec<usize> = read();
        let no = input[0];
        vec[no - 1] = true;
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);


    for i in 0..30 {
        if !vec[i] {
            writeln!(writer, "{}", i + 1).unwrap();
        }
    }
}
