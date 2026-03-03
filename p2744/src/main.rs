use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for char in buffer.chars() {
        if char.is_ascii_lowercase() {
            write!(writer, "{}", char.to_ascii_uppercase()).unwrap();
        } else {
            write!(writer, "{}", char.to_ascii_lowercase()).unwrap();
        }
    }
}
