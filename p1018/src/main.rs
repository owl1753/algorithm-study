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
    let input: Vec<usize> = read();
    let n = input[0];
    let m = input[1];

    let mut map: Vec<Vec<char>> = Vec::new();

    for _ in 0..n {
        let input: Vec<String> = read();
        map.push(input[0].chars().collect());
    }

    let mut result: Option<i32> = None;

    for i in 0..=(n - 8) {
        for j in 0..=(m - 8) {
            for ch in ['W', 'B'] {
                let mut count = 0;

                let mut now = ch;

                for u in 0..8 {
                    for v in 0..8 {
                        let x = i + u;
                        let y = j + v;
                        if now != map[x][y] {
                            count += 1;
                        }
                        now = if now == 'W' { 'B' } else { 'W' }
                    }
                    now = if now == 'W' { 'B' } else { 'W' }
                }

                result = match result {
                    None => Some(count),
                    Some(result) => {
                        if result > count {
                            Some(count)
                        } else {
                            Some(result)
                        }
                    }
                }
            }
        }
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", result.unwrap()).unwrap();
}
