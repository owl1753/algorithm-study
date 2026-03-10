use std::io::{stdin, stdout, BufWriter, Write};

fn read() -> Vec<i32> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<i32>).collect()
}

fn main() {
    let a = read()[0];
    let b = read()[0];
    let c = read()[0];

    let mut result = vec![0; 10];
    let number = (a * b * c).to_string();

    for c in number.chars() {
        let digit = c.to_digit(10).unwrap();
        result[digit as usize] += 1;
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for count in result {
        writeln!(writer, "{}", count).unwrap();
    }
}
