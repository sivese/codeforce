/* https://codeforces.com/problemset/problem/492/B */

use std::collections::VecDeque;
use std::io::{self, BufRead, BufWriter, Read, Write};
use std::str::FromStr;

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
    
    let n: i32 = io.next();
    let l: i32 = io.next();
    
    let mut a: Vec<i32> = (0..n).map(|_| io.next()).collect();
    
    a.sort();

    let mut gap = a[0] * 2;

    for i in 1..n as usize {
        gap = gap.max(a[i] - a[i - 1]);
    }

    let lst = l - a[n as usize - 1];
    gap = gap.max(lst * 2);

    println!("{}", gap as f64 / 2.0);
}pub struct IO<R: BufRead> {
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
    
    let n: i32 = io.next();
    let l: i32 = io.next();
    
    let mut a: Vec<i32> = (0..n).map(|_| io.next()).collect();
    
    a.sort();

    let mut gap = a[0] * 2;

    for i in 1..n as usize {
        gap = gap.max(a[i] - a[i - 1]);
    }

    let lst = l - a[n as usize - 1];
    gap = gap.max(lst * 2);

    println!("{}", gap as f64 / 2.0);
}