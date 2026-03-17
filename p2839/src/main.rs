use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let mut n = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>).next().unwrap();
    let mut result = 0;
    while n > 0 {
        if n % 5 == 0 {
            result += n / 5;
            n = 0;
            break;
        }

        n -= 3;
        result += 1;
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    if n == 0 {
        writeln!(writer, "{}", result).unwrap();
    } else {
        writeln!(writer, "-1").unwrap();
    }
}
