use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let stdout = stdout();

    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{}",  buffer.chars().count() - 1).unwrap();

}
