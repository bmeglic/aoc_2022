use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn priority(ch: &char) -> u32
{
    match *ch {
        'a'..='z' => *ch as u32 - 96,
        'A'..='Z' => *ch as u32 - 65 + 27,
        _ => unreachable!(),
    }
}



pub fn run(file: String) {

    let mut sum = 0;

    if let Ok(lines) = read_lines(&file) {
        for line in lines {
            if let Ok(line) = line {
                let len = line.chars().count();
                let (a, b) = line.split_at(len/2);

                let a: HashSet<char> = HashSet::from_iter(a.chars());
                let b: HashSet<char> = HashSet::from_iter(b.chars());

                let dup = a.intersection(&b).next().unwrap();
                sum += priority(dup);
            }
        }
        println!("Sum: {}", sum);
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
