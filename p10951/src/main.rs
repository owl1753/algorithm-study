use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    loop {
        let a = input.next();

        match a {
            None => break,
            Some(a) => {
                let b = input.next().unwrap();
                writeln!(writer, "{}", a + b).unwrap();
            }
        }
    }
}
