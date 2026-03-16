use std::io::{BufWriter, stdin, stdout, Write};

fn func(n: i32) -> bool {
    let mut cnt = 0;
    let mut tmp = n;
    while tmp != 0 {
        if tmp % 10 == 6 {
            cnt += 1
        } else {
            cnt = 0
        }

        if cnt == 3 {
            return true
        }
        tmp /= 10;
    }
    false
}

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let n = buffer
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>)
        .next()
        .unwrap();
    let mut tmp = 666;
    let mut i = 1;

    loop {
        if n == i {
            break;
        }

        tmp += 1;

        if func(tmp) {
            i += 1;
        }
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", tmp).unwrap();
}
