use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let grade = match buffer.trim() {
        "A+" => 4.3,
        "A0" => 4.0,
        "A-" => 3.7,
        "B+" => 3.3,
        "B0" => 3.0,
        "B-" => 2.7,
        "C+" => 2.3,
        "C0" => 2.0,
        "C-" => 1.7,
        "D+" => 1.3,
        "D0" => 1.0,
        "D-" => 0.7,
        _ => 0.0,
    };

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{:.1}", grade).unwrap();
}
