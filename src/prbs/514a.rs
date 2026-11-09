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

    let raw = io.next::<String>();
    let mut ans = String::new();
    let mut first = true;

    for c in raw.chars() {
        if first && c == '9' {
            ans.push(c);
            first = false;
            continue;
        }
        
        let inv = c.to_digit(10)
            .unwrap();

        if inv < 5 { 
            ans.push(c) 
        }
        else { 
            ans.push(std::char::from_digit(9 - inv, 10).unwrap()) 
        }

        first = false;
    }

    println!("{}", ans);
}