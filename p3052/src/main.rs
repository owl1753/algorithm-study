use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut checks = vec![false; 42];

    for _ in 0..10 {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>);

        let n = input.next().unwrap();

        checks[(n % 42) as usize] = true;
    }

    let mut count = 0;

    for check in checks {
        if check {
            count += 1;
        }
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{}", count).unwrap();
}
