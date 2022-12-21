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

        let (dx, dy) = match (diff.x, diff.y) {
            (0, 0) => (0, 0),
            (0, 1) | (0, -1) | (1, 0) | (-1, 0) => (0, 0),
            (1, 1) | (-1, -1) | (1, -1) | (-1, 1) => (0, 0),
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),

            (2, 1) => (1, 1),
            (2, -1) => (1, -1),
            // need to move to the left diagonally
            (-2, 1) => (-1, 1),
            (-2, -1) => (-1, -1),
            // need to move up/down diagonally
            (1, 2) => (1, 1),
            (-1, 2) => (-1, 1),
            (1, -2) => (1, -1),
            (-1, -2) => (-1, -1),
            _ => (0, 0),
        };

        self.x += dx;
        self.y += dy;

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
            _ => Direction::Right,
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
    let mut head = Coord::new();
    let mut tail = Coord::new();

    let mut visited: HashSet<Coord> = HashSet::new();

    if let Ok(lines) = read_lines(&file) {
        for line in lines {
            if let Ok(line) = line {
                let cmd = Command::new(&line);
                //dbg!(&cmd);

                for _ in 0..cmd.steps {
                    head = head.move_dir(&cmd.direction);
                    tail = tail.follow(&head);
                    //dbg!(&head);

                    visited.insert(tail);
                }
            }
        }
        println!("Visited positions: {}", visited.len());
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
