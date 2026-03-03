use std::io::{BufWriter, Read, Write, stdin, stdout};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut t = input.next().unwrap();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    while t > 0 {
        t -= 1;
        let a = input.next().unwrap();
        let b = input.next().unwrap();

        write!(writer, "{}\n", a + b).unwrap();
    }
}
