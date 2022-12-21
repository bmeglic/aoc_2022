use grid::*;
use itertools::FoldWhile;
use itertools::FoldWhile::{Continue, Done};
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

fn get_coord(grid_size: (usize, usize), index: usize) -> (usize, usize) {
    let r = index / grid_size.0;
    let c = index % grid_size.0;

    (r, c)
}

fn acc_scenic_score(
    forest: &Grid<u8>,
    acc: usize,
    val: &u8,
    x: usize,
    y: usize,
) -> FoldWhile<usize> {
    let val_edge = forest.get(x, y).unwrap();

    if val > val_edge {
        Continue(acc + 1)
    } else {
        Done(acc + 1)
    }
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

        let scenic_score = forest
            .iter()
            .enumerate()
            .map(|(i, val)| {
                let (row, col) = get_coord(forest.size(), i);
                //println!("Processing: {}.{}", row, col);
                if row == 0
                    || col == 0
                    || row == (forest.size().0 - 1)
                    || col == (forest.size().1 - 1)
                {
                    return 0;
                }

                let left = (0..col)
                    .rev()
                    //.inspect(|c| { dbg!(c); })
                    .fold_while(0, |acc, c| acc_scenic_score(&forest, acc, val, row, c))
                    .into_inner();

                let right = (col + 1..forest.size().1)
                    //.inspect(|c| { dbg!(c); })
                    .fold_while(0, |acc, c| acc_scenic_score(&forest, acc, val, row, c))
                    .into_inner();

                let top = (0..row)
                    .rev()
                    //.inspect(|r| { dbg!(r); })
                    .fold_while(0, |acc, r| acc_scenic_score(&forest, acc, val, r, col))
                    .into_inner();

                let bottom = (row + 1..forest.size().0)
                    //.inspect(|r| { dbg!(r); })
                    .fold_while(0, |acc, r| acc_scenic_score(&forest, acc, val, r, col))
                    .into_inner();

                left * right * top * bottom
            })
            .max()
            .unwrap();

        println!("Scenic score: {}", scenic_score);
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
