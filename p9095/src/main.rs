use std::io::{stdin, stdout, BufWriter, Write};

fn read() -> Vec<usize> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<usize>).collect()
}

fn func(n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let mut count = 0;

    if n >= 3 {
        count += func(n - 3);
    }

    if n >= 2 {
        count += func(n - 2);
    }

    if n >= 1 {
        count += func(n - 1);
    }

    count
}

fn main() {
    let mut t = read()[0];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    while t > 0 {
        t -= 1;
        writeln!(writer, "{}", func(read()[0])).unwrap();
    }
}
