use std::io::{stdin, stdout, BufWriter, Write};

fn func(vec: Vec<i32>) -> i32 {
    let mut sum = 0;
    for x in vec {
        sum += x * x;
    }
    sum % 10
}

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let input: Vec<i32> = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>).collect();

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{}", func(input)).unwrap();
}
