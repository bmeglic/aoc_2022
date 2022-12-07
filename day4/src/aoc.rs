use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn new(val: (u32, u32)) -> Section {
        Section {
            start: val.0,
            end: val.1,
        }
    }
    fn is_contained(&self, other: &Self) -> bool {
        if self.start >= other.start && self.end <= other.end {
            true
        } else {
            false
        }
    }
}

pub fn run(file: String) {
    let mut sum = 0;

    if let Ok(lines) = read_lines(&file) {
        for line in lines {
            if let Ok(line) = line {
                let (first, second) = line.split(',').collect_tuple::<(&str, &str)>().unwrap();

                let first = Section::new(
                    first
                        .split('-')
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect_tuple::<(u32, u32)>()
                        .unwrap(),
                );

                let second = Section::new(
                    second
                        .split('-')
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect_tuple::<(u32, u32)>()
                        .unwrap(),
                );

                if first.is_contained(&second) || second.is_contained(&first) {
                    sum += 1;
                }
            }
        }
        println!("Is contained: {}", sum);
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
