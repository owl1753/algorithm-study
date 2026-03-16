use std::io::{stdin, stdout, BufWriter, Write};

fn read() -> Vec<usize> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<usize>).collect()
}

fn main() {
    let mut d = vec![vec![0; 15]; 15];
    for i in 0..=14 {
        for j in 1..=14 {
            d[i][j] = match i {
                0 => j,
                _ => {
                    let mut sum = 0;
                    for k in 1..=j {
                        sum += d[i - 1][k];
                    }
                    sum
                }
            }

        }
    }
    let mut t = read()[0];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    while t > 0 {
        t -= 1;
        let k = read()[0];
        let n = read()[0];
        writeln!(writer, "{}", d[k][n]).unwrap();
    }
}
