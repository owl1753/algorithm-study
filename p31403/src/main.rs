use std::io::{stdin, stdout, BufWriter, Write};

fn read() -> Vec<String> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<String>).collect()
}

fn main() {
    let input = read();
    let a = &input[0];
    let input = read();
    let b = &input[0];
    let input = read();
    let c = &input[0];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}",  a.parse::<i32>().unwrap() + b.parse::<i32>().unwrap() - c.parse::<i32>().unwrap()).unwrap();
    writeln!(writer, "{}", (a.to_owned() + b).parse::<i32>().unwrap() - c.parse::<i32>().unwrap()).unwrap();
}
