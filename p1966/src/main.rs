use std::collections::VecDeque;
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
    let mut t: usize = read()[0];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    while t > 0 {
        let mut queue = VecDeque::new();
        t -= 1;
        let input: Vec<usize> = read();
        let n = input[0];
        let m = input[1];
        let input: Vec<i32> = read();
        for i in 0..n {
            queue.push_back((i, input[i]));
        }
        let mut count = 0;
        while !queue.is_empty() {

            let front =queue.pop_front().unwrap();
            let mut flag = false;
            for el in queue.iter() {
                if el.1 > front.1 {
                    flag = true;
                    break;
                }
            }
            if flag {
                queue.push_back(front);
            } else {
                count += 1;
                if m == front.0 {
                    writeln!(writer, "{}", count).unwrap();
                    break;
                }
            }
        }
    }
}
