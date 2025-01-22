use std::io::{self, BufRead, Write};

fn is_valid_x(x: u64, a: &Vec<u64>) -> bool {
    for &ai in a {
        if x % ai == 0 || ai % x == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    let mut input = stdin.lock().lines();

    let t: usize = input.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let n: usize = input.next().unwrap().unwrap().trim().parse().unwrap();
        let a: Vec<u64> = input
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if a.contains(&1) {
            writeln!(writer, "-1").unwrap();
            continue;
        }

        let mut x = 2;
        while !is_valid_x(x, &a) {
            x += 1;
        }
        writeln!(writer, "{}", x).unwrap();
    }
}
