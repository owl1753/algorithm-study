use std::io::{BufWriter, Write, stdin, stdout};

fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    loop {
        let mut buffer = String::new();
        let mut stack: Vec<char> = Vec::new();
        stdin().read_line(&mut buffer).unwrap();
        let mut input = buffer.trim_end();
        let mut flag = true;
        if input == "." {
            break;
        }
        for ch in input.chars() {
            match ch {
                '[' | '(' => stack.push(ch),
                ']' | ')' => {
                    let top = stack.pop();

                    match top {
                        Some(top) => {
                            if (ch == ']' && top != '[') || (ch == ')' && top != '(') {
                                flag = false;
                                break;
                            }
                        }
                        None => {
                            flag = false;
                            break;
                        }
                    }
                }
                _ => (),
            }
        }
        if !stack.is_empty() {
            flag = false;
        }

        if flag {
            writeln!(writer, "yes").unwrap();
        } else {
            writeln!(writer, "no").unwrap();
        }
    }
}
