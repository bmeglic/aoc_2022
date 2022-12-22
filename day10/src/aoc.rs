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

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    AddX(i64),
}

impl Instruction {
    fn new(input: &str) -> Instruction {
        if input == "noop" {
            Instruction::Noop
        } else if input.starts_with("addx") {
            let (_, val) = input.split_once(' ').unwrap();
            let val: i64 = val.parse().unwrap();
            Instruction::AddX(val)
        } else {
            unreachable!();
        }
    }
}

fn print_screen(input: &[bool], screen_width: usize) {
    input.iter().enumerate().for_each(|(i, c)| {
        if *c {
            print!("#");
        } else {
            print!(" ");
        }

        if i % screen_width == (screen_width - 1) {
            println!();
        }
    });
}

pub fn run(file: String) {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut cycle: i64 = 1;
    let mut x: i64 = 1;
    let mut signal_strength: i64 = 0;

    let mut screen: Vec<bool> = vec![false; 40 * 6];

    if let Ok(lines) = read_lines(&file) {
        for line in lines.flatten() {
            instructions.push(Instruction::new(&line[..]));
        }

        let mut instr_iter = instructions.iter();
        let mut instr: Option<&Instruction> = None;
        let mut instr_cycle = 0;

        let cycles_to_match = vec![20, 60, 100, 140, 180, 220];
        loop {
            //println!("Cycle: {}", cycle);

            if instr.is_none() {
                instr = instr_iter.next();
                if instr.is_none() {
                    break;
                }

                if let Instruction::AddX(_) = *instr.unwrap() {
                    instr_cycle = 2;
                }
            }

            if cycles_to_match.contains(&cycle) {
                signal_strength += cycle * x;
            }

            if (((cycle - 1) % 40) >= (x - 1)) && (((cycle - 1) % 40) <= (x + 1)) {
                println!("Cycle: {}", (cycle - 1) % 40);
                screen[(cycle - 1) as usize] = true;
            }

            match *instr.unwrap() {
                Instruction::Noop => instr = None,
                Instruction::AddX(val) => {
                    instr_cycle -= 1;
                    if instr_cycle == 0 {
                        x += val;
                        instr = None;
                        //println!("New X: {}", X);
                    }
                }
            }

            cycle += 1;
        }

        //dbg!(&instructions);
        print_screen(&screen, 40);
        println!("Signal strength: {}", signal_strength);
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
