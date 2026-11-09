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

    let t= io.next::<usize>();

    for _ in 0..t {
        /* 1352c */
        let n = io.next::<i32>();
        let k = io.next::<i32>();
    
        let mul = k / (n - 1);

        let ans = if mul * (n - 1) == k {
            mul * n - 1
        } else {
            mul * n + (k - mul * (n - 1))
        };

        println!("{}", ans);
    }

}