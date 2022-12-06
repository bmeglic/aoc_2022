use std::collections::HashSet;
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

fn priority(ch: &char) -> u32 {
    match *ch {
        'a'..='z' => *ch as u32 - 96,
        'A'..='Z' => *ch as u32 - 65 + 27,
        _ => unreachable!(),
    }
}

pub fn run(file: String) {
    let mut sum = 0;

    if let Ok(mut lines) = read_lines(&file) {
        while let (Some(elf_a), Some(elf_b), Some(elf_c)) =
            (lines.next(), lines.next(), lines.next())
        {
            let a: HashSet<char> = HashSet::from_iter(elf_a.unwrap().chars());
            let b: HashSet<char> = HashSet::from_iter(elf_b.unwrap().chars());
            let c: HashSet<char> = HashSet::from_iter(elf_c.unwrap().chars());

            let dup: HashSet<char> = a.intersection(&b).copied().collect();
            let dup = dup.intersection(&c).next().unwrap();

            sum += priority(dup);
        }
        println!("Sum: {}", sum);
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
