use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", buffer).unwrap();
}
