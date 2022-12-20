use grid::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_coord(grid_size: (usize, usize), index: &usize) -> (usize, usize) {
    let r = index / grid_size.0;
    let c = index % grid_size.0;

    (r, c)
}


pub fn run(file: String) {
    let mut forest: Grid<u8> = Grid::init(0, 0, 0);

    if let Ok(lines) = read_lines(&file) {
        for line in lines {
            if let Ok(line) = line {
                let treeline: Vec<u8> = line
                    .chars()
                    .map(|n| n.to_digit(10).unwrap() as u8)
                    .collect();
                forest.push_row(treeline);
            }
        }

        //dbg!(&forest);

        let visible = forest
            .iter()
            .enumerate()
            .filter(|(i, val)| {
                let (row, col) = get_coord(forest.size(), i);
                //println!("Processing: {}.{}", row, col);
                if row == 0
                    || col == 0
                    || row == (forest.size().0 - 1)
                    || col == (forest.size().1 - 1)
                {
                    return true;
                }

                let highest = (0..row)
                    .into_iter()
                    .cartesian_product(col..col+1)
                    .map(|(x,y)| forest.get(x,y).unwrap())
                    .max()
                    .unwrap();
                if *val > highest {
                    return true;
                }

                let highest = (row+1..forest.size().0)
                    .into_iter()
                    .cartesian_product(col..col+1)
                    .map(|(x,y)| forest.get(x,y).unwrap())
                    .max()
                    .unwrap();
                if *val > highest {
                    return true;
                }

                let highest = (row..row+1)
                    .into_iter()
                    .cartesian_product(0..col)
                    .map(|(x,y)| forest.get(x,y).unwrap())
                    .max()
                    .unwrap();
                if *val > highest {
                    return true;
                }

                let highest = (row..row+1)
                    .into_iter()
                    .cartesian_product(col+1..forest.size().1)
                    .map(|(x,y)| forest.get(x,y).unwrap())
                    .max()
                    .unwrap();
                if *val > highest {
                    return true;
                }

                false
            })
            .count();

        println!("Visible trees: {}", visible);
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
