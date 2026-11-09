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

    //1520d
    let t = io.next::<usize>();

    for _ in 0..t {
        let n = io.next::<usize>();
        let mut arr = (0..n).map(|x| (io.next::<i32>(), x))
            .collect::<Vec<_>>();

        let mut ans = 0;

        while arr.len() > 1 {
            let mut nxt: Vec<(i32, usize)> = Vec::new();
            let mut cnt = 0;
    
            for i in 1..arr.len() {
                let gap = (arr[i].1 - arr[0].1) as i32; // index gap
    
                if gap == (arr[i].0 - arr[0].0) {  //compare the index gap with value difference
                    cnt += 1; 
                }
                else {
                    nxt.push(arr[i]);
                }
            }
    
            ans += cnt * (cnt + 1) / 2;
            arr = nxt;
        }

        println!("{}", ans);
    }
}