use std::io::{BufWriter, Write, stdin, stdout};
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
    let mut m = read::<usize>()[0];
    let mut s = vec![false; 21];

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    while m > 0 {
        m -= 1;
        let input = read::<String>();
        let query = &input[0];
        match query.as_str() {
            "add" => {
                let x = input[1].parse::<usize>().unwrap();
                s[x] = true;
            }
            "remove" => {
                let x = input[1].parse::<usize>().unwrap();
                s[x] = false;
            }
            "check" => {
                let x = input[1].parse::<usize>().unwrap();
                writeln!(writer, "{}", s[x] as i32).unwrap();
            }
            "toggle" => {
                let x = input[1].parse::<usize>().unwrap();
                if s[x] {
                    s[x] = false;
                } else {
                    s[x] = true;
                }
            }
            "all" => {
                for i in 1..=20 {
                    s[i] = true;
                }
            }
            "empty" => {
                for i in 1..=20 {
                    s[i] = false;
                }
            }
            _ => (),
        }
    }
}
