use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
pub struct Data {
    pub stacks: Vec<VecDeque<char>>,
}

impl Data {
    pub fn new(reader: &mut BufReader<File>) -> ::std::result::Result<Data, Error> {
        const STACK_WIDTH: usize = 4;
        let mut stacks: Vec<VecDeque<char>> = vec![];
        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            if stacks.is_empty() {
                stacks = vec![VecDeque::<char>::new(); line.len() / STACK_WIDTH];
            }

            if line.chars().all(|ch| ch.is_numeric() || ch.is_whitespace()) {
                break;
            }

            for (i, ch) in line.chars().enumerate() {
                if ch.is_ascii_alphabetic() {
                    let index = i / STACK_WIDTH;
                    if index >= stacks.len() {
                        return Err(Error::new(std::io::ErrorKind::Other, "Invalid input"));
                    }
                    stacks[index].push_back(ch);
                }
            }
        }

        Ok(Data { stacks })
    }
}
