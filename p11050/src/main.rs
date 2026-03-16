use std::io::{stdin, stdout, BufWriter, Write};

fn fact(n: i32) -> i32 {
    let mut tmp = 1;
    for i in 1..=n {
        tmp *= i;
    }
    tmp
}

fn read() -> Vec<i32> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<i32>).collect()
}

fn main() {
    let input = read();
    let n = input[0];
    let k = input[1];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", fact(n) / (fact(n - k) * fact(k))).unwrap();
}
