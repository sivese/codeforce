/* https://codeforces.com/problemset/problem/2268/A */

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

    let t = io.next::<usize>();

    for _ in 0..t {
        let n = io.next::<i64>();
        let k = io.next::<i64>();

        let mut a = (0..n).map(|_| io.next::<i64>())
            .collect::<Vec<_>>();

        let mut ans:i64 = 0;

        
        loop {
            let len = a.len() as i64;
            let r = len - k + 1;
            
            let one = a[k as usize - 1];
            let two = a[r as usize - 1];

            ans += if one > two { 
                a.remove(k as usize - 1);
                one 
            }
            else { 
                a.remove(r as usize - 1);
                two 
            };

            if a.len() < k as usize { break; }
        }

        println!("{}", ans);
    }
}