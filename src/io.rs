use std::io::{self, BufWriter, Read, Write};
use std::str::FromStr;

pub struct IO {
    input: String,
    pos: usize,
}

impl IO {
    pub fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { input, pos: 0 }
    }

    pub fn next<T: FromStr>(&mut self) -> T {
        let bytes = self.input.as_bytes();

        while self.pos < bytes.len() && bytes[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }

        let start = self.pos;

        while self.pos < bytes.len() && !bytes[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }

        assert!(start < self.pos, "Unexpected end of input");

        self.input[start..self.pos]
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse input"))
    }
}