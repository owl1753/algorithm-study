use std::collections::VecDeque;
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
    let mut n: usize = read()[0];
    let mut queue = VecDeque::new();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    while n > 0 {
        n -= 1;
        let input: Vec<String> = read();
        let query = &input[0];
        match query.as_str() {
            "push" => {
                let i = input[1].parse::<i32>().unwrap();
                queue.push_back(i);
            }
            "pop" => {
                let front = queue.pop_front();
                match front {
                    Some(front) => writeln!(writer, "{}", front).unwrap(),
                    None => writeln!(writer, "-1").unwrap(),
                }
            }
            "size" => writeln!(writer, "{}", queue.len()).unwrap(),
            "empty" => writeln!(writer, "{}", queue.is_empty() as i32).unwrap(),
            "front" => {
                let front = queue.front();
                match front {
                    Some(front) => writeln!(writer, "{}", *front).unwrap(),
                    None => writeln!(writer, "-1").unwrap(),
                }
            }
            "back" => {
                let back = queue.back();
                match back {
                    Some(back) => writeln!(writer, "{}", *back).unwrap(),
                    None => writeln!(writer, "-1").unwrap(),
                }
            },
            _ => (),
        }
    }
}
