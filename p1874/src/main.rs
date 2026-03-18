use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn read<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<T>).collect()
}

fn main() {
    let n = read::<usize>()[0];
    let mut stack = Vec::new();
    let mut result = Vec::new();
    let mut anchor = 1;
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for _ in 0..n {
        let query = read::<i32>()[0];

        while anchor <= query {
            stack.push(anchor);
            result.push("+");
            anchor += 1;
        }

        let top = stack.pop().unwrap();
        result.push("-");

        if top != query {
            writeln!(writer, "NO").unwrap();
            return;
        }
    }

    for el in result {
        writeln!(writer, "{}", el).unwrap();
    }
}
