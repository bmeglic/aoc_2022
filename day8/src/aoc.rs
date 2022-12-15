use grid::*;
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
                if row == 0
                    || col == 0
                    || row == (forest.size().0 - 1)
                    || col == (forest.size().1 - 1)
                {
                    return true;
                }

                let mut highest = 0;
                for r in 0..row {
                    let val_edge = forest.get(r, col).unwrap();
                    if *val_edge > highest {
                        highest = *val_edge;
                    }
                }
                if **val > highest {
                    return true;
                }

                highest = 0;
                for r in row + 1..forest.size().0 {
                    let val_edge = forest.get(r, col).unwrap();
                    if *val_edge > highest {
                        highest = *val_edge;
                    }
                }
                if **val > highest {
                    return true;
                }

                highest = 0;
                for c in 0..col {
                    let val_edge = forest.get(row, c).unwrap();
                    if *val_edge > highest {
                        highest = *val_edge;
                    }
                }
                if **val > highest {
                    return true;
                }

                highest = 0;
                for c in col + 1..forest.size().1 {
                    let val_edge = forest.get(row, c).unwrap();
                    if *val_edge > highest {
                        highest = *val_edge;
                    }
                }
                if **val > highest {
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
