use std::io::{stdin, stdout, BufWriter, Write};

const MAX: usize = 1000000;

fn main() {
    let mut buffer = String::new();
    let mut vec = vec![true; MAX + 1];
    for i in 1..=MAX {
        if !vec[i] {
            continue;
        }

        match i {
            1 => vec[i] = false,
            n => {
                let mut j = 2 * n;
                while j <= MAX {
                    vec[j] = false;
                    j += n;
                }
            }
        }
    }
    stdin().read_line(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut m = input.next().unwrap();
    let n = input.next().unwrap();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    while m <= n {
        if vec[m] {
            writeln!(writer, "{}", m).unwrap();
        }
        m += 1;
    }
}
