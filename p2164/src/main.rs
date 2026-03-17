use std::collections::LinkedList;
use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>).next().unwrap();
    let mut list = LinkedList::new();
    for i in 1..=n {
        list.push_back(i);
    }
    while list.len() != 1 {
        list.pop_front();
        let tmp = list.pop_front().unwrap();
        list.push_back(tmp);
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    writeln!(writer, "{}", list.pop_front().unwrap()).unwrap();
}
