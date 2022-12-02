use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Elf {
    calories: u32,
}


pub fn run(file: String) {

    let mut elves: Vec<Elf> = Vec::new();

    let mut elf = Elf { calories: 0 };

    if let Ok(lines) = read_lines(&file) {
        for line in lines {
            if let Ok(line) = line {
                if line == "" {
                    elves.push(elf);
                    elf = Elf { calories : 0 };
                }
                else {
                    let calories = line.parse::<u32>().unwrap_or(0);
                    elf.calories += calories;
                }
            }
        }
    }
    else {
        println!("Could not open/read file: {}", &file);
    }

    let mut max_calories = 0;

    for elf in elves.iter() {
        if elf.calories > max_calories {
            max_calories = elf.calories;
        }
    }

    println!("Elf with most calories is carrying: {} calories", max_calories);
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
