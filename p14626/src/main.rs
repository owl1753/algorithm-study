use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let chars: Vec<char> = buffer.chars().collect();
    let mut weight = 1;
    let mut sum = 0;
    let m = chars[12].to_digit(10).unwrap();
    let m = if m == 0 { 10 } else { m };

    for i in 0..12 {
        sum += match chars[i] {
            '*' => {
                if i % 2 == 0 {
                    weight = 1;
                } else {
                    weight = 3;
                }
                0
            }
            char => {
                if i % 2 == 0 {
                    char.to_digit(10).unwrap()
                } else {
                    3 * char.to_digit(10).unwrap()
                }
            }
        };
        sum %= 10;
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for i in 0..=9 {
        if (weight * i + sum) % 10 == 10 - m {
            writeln!(writer, "{}", i).unwrap();
        }
    }
}