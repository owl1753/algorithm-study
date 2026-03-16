use std::cmp::min;
use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>).next().unwrap();
    let mut count_2 = 0;
    let mut count_5 = 0;

    for i in 1..=n {
        let mut tmp = i;

        loop {
            if tmp % 2 == 0 {
                tmp /= 2;
                count_2 += 1;
            } else {
                break;
            }
        }

        loop {
            if tmp % 5 == 0 {
                tmp /= 5;
                count_5 += 1;
            } else {
                break;
            }
        }
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", min(count_2, count_5)).unwrap();
}
