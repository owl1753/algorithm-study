use std::io::{BufWriter, Write, stdin, stdout};

fn read() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    loop {
        let number = read();
        if number.trim() == "0" {
            break;
        }
        let number: Vec<char> = number.trim().chars().collect();
        let mut flag = true;
        for i in 0..number.len() {
            if number[i] != number[number.len() - i - 1] {
                flag = false;
                break;
            }
        }
        if flag {
            writeln!(writer, "yes").unwrap();
        } else {
            writeln!(writer, "no").unwrap();
        }
    }
}
