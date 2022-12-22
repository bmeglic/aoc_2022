use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Sub;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new() -> Coord {
        Coord { x: 0, y: 0 }
    }

    fn move_dir(mut self, dir: &Direction) -> Self {
        match dir {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
        self
    }

    fn follow(mut self, other: &Self) -> Self {
        let diff = *other - self;

        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            self.x += diff.x.signum();
            self.y += diff.y.signum();
        }

        //dbg!(&diff);
        self
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Self) -> Self {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn parse(input: &str) -> Direction {
        match input {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    steps: u32,
}

impl Command {
    fn new(input: &str) -> Command {
        let mut split = input.split(" ");

        Command {
            direction: Direction::parse(split.next().unwrap()),
            steps: split.next().unwrap().parse().unwrap(),
        }
    }
}

pub fn run(file: String) {
    let mut knots: Vec<Coord> = vec![Coord::new(); 10];
    let mut visited: HashSet<Coord> = HashSet::new();

    if let Ok(lines) = read_lines(&file) {
        for line in lines {
            if let Ok(line) = line {
                let cmd = Command::new(&line);
                //dbg!(&cmd);

                for _ in 0..cmd.steps {
                    knots[0] = knots[0].move_dir(&cmd.direction);
                    //dbg!(&knots[0]);

                    for i in 1..knots.len() {
                        knots[i] = knots[i].follow(&knots[i - 1]);
                        //dbg!(&knots[i]);
                    }

                    visited.insert(knots[knots.len() - 1]);
                }
            }
        }
        println!("Visited positions: {}", visited.len());
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
