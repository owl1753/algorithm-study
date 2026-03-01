use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut t = input.next().unwrap();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    while t > 0 {
        t -= 1;

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

        let a = input.next().unwrap();
        let b = input.next().unwrap();

        writeln!(writer, "{}", a + b).unwrap();
    }
}
