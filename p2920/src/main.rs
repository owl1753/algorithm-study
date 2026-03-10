use std::io::{stdin, stdout, BufWriter, Write};

enum State {
    Ascending,
    Descending,
    Mixed,
}

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut before: Option<i32> = None;
    let mut state: Option<State> = None;

    while let Some(note) = input.next() {
        before = match before {
            None => { Some(note) },
            Some(before) => {
                state = match &state {
                    None => {
                        if note < before {
                            Some(State::Descending)
                        } else {
                            Some(State::Ascending)
                        }
                    }
                    Some(State::Ascending) => {
                        if note < before {
                            Some(State::Mixed)
                        } else {
                            Some(State::Ascending)
                        }
                    }
                    Some(State::Descending) => {
                        if note > before {
                            Some(State::Mixed)
                        } else {
                            Some(State::Descending)
                        }
                    }
                    Some(State::Mixed) => Some(State::Mixed),
                };
                Some(note)
            }
        }
    }

    let stdout = stdout();

    let mut writer = BufWriter::new(stdout.lock());

    match state.unwrap() {
        State::Ascending => {
            writeln!(writer, "{}", "ascending").unwrap();
        },
        State::Descending => {
            writeln!(writer, "{}", "descending").unwrap();
        },
        State::Mixed => {
            writeln!(writer, "{}", "mixed").unwrap();
        }
    }
}
