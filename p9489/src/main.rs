use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let score = input.next().unwrap();
    let grade: char;

    if score >= 90 {
        grade = 'A';
    } else if score >= 80 {
        grade = 'B';
    } else if score >= 70 {
        grade = 'C';
    } else if score >= 60 {
        grade = 'D';
    } else {
        grade = 'F';
    }

    println!("{}", grade);
}
