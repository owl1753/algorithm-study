use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>);
        let x = input.next().unwrap();
        let y = input.next().unwrap();
        let z = input.next().unwrap();

        if x == 0 && y == 0 && z == 0 {
            break;
        }

        let result =
            if (x * x + y * y == z * z) | (x * x + z * z == y * y) | (z * z + y * y == x * x) {
                "right"
            } else {
                "wrong"
            };

        writeln!(writer, "{}", result).unwrap();
    }
}
