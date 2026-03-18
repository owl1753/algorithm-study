use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let k = input.next().unwrap();
    let mut vec = Vec::new();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for i in 1..=n {
        vec.push(i);
    }
    let mut idx: usize = 0;
    let mut result = Vec::new();
    while !vec.is_empty() {
        idx += (k - 1);
        idx %= vec.len();
        result.push(vec.remove(idx).to_string());
    }
    write!(writer, "<").unwrap();
    write!(writer, "{}", result.join(", ")).unwrap();
    write!(writer, ">").unwrap();
}
