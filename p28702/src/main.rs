use std::io::{stdin, stdout, BufWriter, Write};

fn read() -> Vec<String> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().flat_map(str::parse::<String>).collect()
}

fn main() {
    let input = read();
    let a = &input[0];
    let input = read();
    let b = &input[0];
    let input = read();
    let c = &input[0];

    let mut n: i32 = 0;

    if a != "Fizz" && a != "Buzz" && a != "FizzBuzz" {
        n = a.parse::<i32>().unwrap() + 3;
    } else if b != "Fizz" && b != "Buzz" && b != "FizzBuzz" {
        n = b.parse::<i32>().unwrap() + 2;
    } else if c != "Fizz" && c != "Buzz" && c != "FizzBuzz" {
        n = c.parse::<i32>().unwrap() + 1;
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    if n % 3 == 0 && n % 5 == 0 {
        writeln!(writer, "FizzBuzz").unwrap();
    } else if n % 3 == 0 {
        writeln!(writer, "Fizz").unwrap();
    } else if n % 5 == 0 {
        writeln!(writer, "Buzz").unwrap();
    } else {
        writeln!(writer, "{}", n).unwrap();
    }
}
