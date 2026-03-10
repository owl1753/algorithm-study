use std::io::{BufWriter, Write, stdin, stdout};

fn main() {
    let mut alphas: Vec<i32> = vec![-1; 26];

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let chars = buffer.trim().chars();

    for (i, char) in chars.enumerate() {
        let idx = (char as usize) - ('a' as usize);
        if alphas[idx] == -1 {
            alphas[idx] = i as i32;
        }
    }

    for i in 0..26 {
        write!(writer, "{} ", alphas[i]).unwrap();
    }
}
