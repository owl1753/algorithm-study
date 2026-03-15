use std::io::{stdin, stdout, BufWriter, Write};

fn read() -> Vec<i32> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>)
        .collect()
}

fn main() {
    let input = read();
    let mut result: Option<i32> = None;
    let n = input[0] as usize;
    let m = input[1];
    let cards: Vec<i32> = read();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let tmp = cards[i] + cards[j] + cards[k];
                result = match result {
                    None => {
                        if tmp <= m{
                            Some(tmp)
                        } else {
                            None
                        }
                    },
                    Some(result) => {
                        if result < tmp && tmp <= m {
                            Some(tmp)
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
