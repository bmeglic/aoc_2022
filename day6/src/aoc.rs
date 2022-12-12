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

pub fn run(file: String) {
    if let Ok(lines) = read_lines(&file) {
        for line in lines {
            if let Ok(line) = line {
                let sop = line
                    .as_bytes()
                    .windows(4)
                    .position(|x| {
                        let map: HashSet<&u8> = HashSet::from_iter(x);
                        map.len() == 4
                    })
                    .expect("Index not found")
                    + 4;
                //println!("Found first Start Of Packet marker after character: {}", sop);

                let som = line[sop..]
                    .as_bytes()
                    .windows(14)
                    .position(|x| {
                        let map: HashSet<&u8> = HashSet::from_iter(x);
                        map.len() == 14
                    })
                    .expect("Index not found")
                    + 14
                    + sop;
                println!(
                    "Found first Start Of Message marker after character: {}",
                    som
                );
            }
        }
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
