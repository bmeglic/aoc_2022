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
        for line in lines.flatten() {
            println!("{}", line);
        }
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
