use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut max = 0;
    let mut argmax = 0;
    for i in 1..=9 {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let mut buffer = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>);
        let n = buffer.next().unwrap();
        if max < n {
            max = n;
            argmax = i;
        }
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}\n{}", max, argmax).unwrap();
}
