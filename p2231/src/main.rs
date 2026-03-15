use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>).next().unwrap();
    let mut result = 0;

    for i in 1..n {
        let mut tmp = i;
        let mut sum = tmp;

        while tmp != 0 {
            sum += tmp % 10;
            tmp /= 10;
        }



        if sum == n {
            result = i;
            break;
        }
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", result).unwrap();
}
