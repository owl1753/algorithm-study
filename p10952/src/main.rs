use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
        let a = input.next().unwrap();
        let b = input.next().unwrap();

        if a == 0 && b == 0 {
            break;
        }

        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{}", a + b).unwrap();
    }
}
