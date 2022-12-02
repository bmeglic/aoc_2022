use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Elf {
    calories: u32,
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.calories).cmp(&(other.calories))
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        (self.calories) == (other.calories)
    }
}

impl Eq for Elf {}

impl Elf {
    fn get_elves_with_max_calories(elves: Vec<Elf>, num: u32) -> Vec<Elf> {
        let mut elves_max: Vec<Elf> = Vec::new();
        let mut elves_tmp = elves.clone();

        let mut i = 0;
        while i < num {
            let index = elves_tmp
                .iter()
                .enumerate()
                .max_by(|(_, x), (_, y)| x.cmp(y))
                .map(|(index, _)| index)
                .unwrap();
            dbg!(&index);
            let max_elf = elves_tmp.remove(index);

            elves_max.push(max_elf);
            i += 1;
        }

        elves_max
    }

    fn sum_calories(elves: Vec<Elf>) -> u32 {
        let mut calories = 0;

        for elf in elves.iter() {
            calories += elf.calories;
        }

        calories
    }
}

pub fn run(file: String) {
    let mut elves: Vec<Elf> = Vec::new();

    let mut elf = Elf { calories: 0 };

    if let Ok(lines) = read_lines(&file) {
        for line in lines {
            if let Ok(line) = line {
                if line == "" {
                    elves.push(elf);
                    elf = Elf { calories: 0 };
                } else {
                    let calories = line.parse::<u32>().unwrap_or(0);
                    elf.calories += calories;
                }
            }
        }
        elves.push(elf);
    } else {
        println!("Could not open/read file: {}", &file);
    }

    let mut max_calories = 0;

    for elf in elves.iter() {
        if elf.calories > max_calories {
            max_calories = elf.calories;
        }
    }

    let elves_max_calories = Elf::get_elves_with_max_calories(elves, 3);
    let max_calories = Elf::sum_calories(elves_max_calories);

    println!(
        "Elf with most calories is carrying: {} calories",
        max_calories
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
