use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let mut t = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>).next().unwrap();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    while t > 0 {
        t -= 1;
        let mut count = 0;
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let input = buffer.trim_end();

        for ch in input.chars() {
            match ch {
                '(' => count += 1,
                ch if ch == ')' && count > 0 => count -= 1,
                ch if ch == ')' && count <= 0 => {
                    count -= 1;
                    break;
                }
                _ => ()
            }
        }

        if count == 0 {
            writeln!(writer, "YES").unwrap();
        } else {
            writeln!(writer, "NO").unwrap();
        }
    }
}
