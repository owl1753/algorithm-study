use std::io::{stdin, stdout, BufWriter, Write};

struct Solve {
    check: Vec<bool>,
    graph: Vec<Vec<usize>>,
}

impl Solve {
    fn new(n: usize) -> Solve {
        Solve {
            graph: vec![Vec::new(); n + 1],
            check: vec![false; n + 1],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.graph[u].push(v);
        self.graph[v].push(u);
    }

    fn func(&mut self, n: usize) -> i32 {
        if self.check[n] {
            return 0;
        }

        let mut count = 1;

        self.check[n] = true;

        for i in 0..self.graph[n].len() {
            count += self.func(self.graph[n][i]);
        }

        count
    }
}

fn read() -> Vec<usize> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer
        .split_ascii_whitespace()
        .flat_map(str::parse::<usize>)
        .collect()
}

fn main() {
    let n = read()[0];
    let m = read()[0];

    let mut solve = Solve::new(n);

    for _ in 0..m {
        let input = read();
        let u = input[0];
        let v = input[1];
        solve.add_edge(u, v);
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", solve.func(1) - 1).unwrap();
}
