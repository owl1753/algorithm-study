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
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let mut stack = Vec::new();

    while n > 0 {
        n -= 1;
        let input: Vec<String> = read();
        let query = &input[0];
        match query.as_str() {
            "push" => {
                let i = input[1].parse::<i32>().unwrap();
                stack.push(i);
            }
            "pop" => {
                let top = stack.pop();
                match top {
                    Some(top) => writeln!(writer, "{}", top).unwrap(),
                    None => writeln!(writer, "-1").unwrap(),
                }
            }
            "size" => writeln!(writer, "{}", stack.len()).unwrap(),
            "empty" => writeln!(writer, "{}", stack.is_empty() as i32).unwrap(),
            "top" => {
                if stack.is_empty() {
                    writeln!(writer, "-1").unwrap();
                } else {
                    let top = stack[stack.len() - 1];
                    writeln!(writer, "{}", top).unwrap();
                }
            }
            _ => (),
        }
    }
}