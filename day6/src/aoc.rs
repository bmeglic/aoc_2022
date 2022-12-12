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
                let wnd = line
                    .as_bytes()
                    .windows(4)
                    .position(|x| {
                        x[0] != x[1]
                            && x[0] != x[2]
                            && x[0] != x[3]
                            && x[1] != x[2]
                            && x[1] != x[3]
                            && x[2] != x[3]
                    })
                    .expect("Index not found");
                println!("Found first marker after character: {}", wnd + 4);
            }
        }
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
