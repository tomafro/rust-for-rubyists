use std::env::args;
use std::fs::File;
use std::io::{ BufReader, BufRead };
use std::collections::{ HashSet, VecDeque };

use regex::{ Regex, escape };

pub struct Buffer {
    capacity: usize,
    buffer: VecDeque<String>
}

impl Buffer {
    pub fn new(capacity: usize) -> Buffer {
        let buffer = VecDeque::with_capacity(capacity);
        Buffer {
            capacity,
            buffer
        }
    }

    pub fn append(&mut self, line: String) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }

        self.buffer.push_back(line);
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<String> {
        self.buffer.iter()
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let rx = Regex::new(&escape(&args[1])).unwrap();

    let file = BufReader::new(File::open(&args[2]).unwrap());

    let mut matches: HashSet<String> = HashSet::new();
    let mut buffer = Buffer::new(1000);
    let splitter = Regex::new(r"^\[(?P<context>[^\]]+)\](?: \[[^\]]+\])? \[(?P<id>[a-z0-9]+…|[a-f0-9-]+)\] (?P<rest>.*)").unwrap();

    let mut lines = file.lines();

    let mut count: usize = 0;

    while let Some(Ok(line)) = lines.next() {
        count += 1;

        if let Some(captures) = splitter.captures(&line) {
            let id = captures.name("id").unwrap().as_str();

            if matches.contains(id) {
                println!("{}", line);
            }
            else if rx.is_match(&line) {
                matches.insert(id.to_string());

                let mut previous_lines = buffer.iter();
                while let Some(previous) = previous_lines.next() {
                    if let Some(captures) = splitter.captures(&previous) {
                        let previous_id = captures.name("id").unwrap().as_str();
                        if previous_id == id {
                            println!("{}", previous);
                        }
                    }
                }

                println!("{}", line);
            }
        }

        buffer.append(line);
    }

    println!("Searched through {} lines", count);
}
