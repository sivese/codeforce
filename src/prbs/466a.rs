// https://codeforces.com/problemset/problem/466/A

use std::collections::VecDeque;
use std::io::{self, BufRead, BufWriter, Read, Write};
use std::str::FromStr;

pub struct IO<R: BufRead> {
    reader: R,
    line: String,
    tokens: VecDeque<String>,
}

impl<R: BufRead> IO<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            line: String::new(),
            tokens: VecDeque::new(),
        }
    }

    pub fn next<T: FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.tokens.pop_front() {
                return token
                    .parse()
                    .unwrap_or_else(|_| panic!("Failed to parse input"));
            }

            self.line.clear();

            let bytes_read = self.reader.read_line(&mut self.line).unwrap();
            assert!(bytes_read > 0, "Unexpected end of input");

            self.tokens
                .extend(self.line.split_whitespace().map(str::to_owned));
        }
    }
}

fn main() {
    let mut io = IO::new(io::stdin().lock());

    let n = io.next::<i32>();
    let m = io.next::<i32>();
    let a = io.next::<i32>();
    let b = io.next::<i32>();

    let cost = b / m;
    
    if cost >= a {
        println!("{}", a * n);
    } else {
        let mut ans = (n / m) * b;
        let rem = n % m;
        ans += if rem * a > b { b } else { rem * a };
        println!("{}", ans);
    }
}