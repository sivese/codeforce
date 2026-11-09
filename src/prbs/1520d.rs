use std::collections::{HashMap, VecDeque};
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

    //1520d
    let t = io.next::<usize>();

    for _ in 0..t {
        let n = io.next::<usize>();
        let mut arr = (0..n).map(|_| io.next::<i64>())
            .collect::<Vec<_>>();
        
        let mut map = HashMap::<i64, i64>::new();
        let mut ans = 0;

        for i in 0..n {
            let gap = arr[i] - i as i64;
            
            let value = map.entry(gap).or_insert(0);
            *value += 1;
        }

        for (k, v) in map.iter() {
            ans += v * (v - 1) / 2;
        }

        println!("{}", ans);
    }
}