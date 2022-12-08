use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::VecDeque;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Command {
    num: u32,
    source: usize,
    destination: usize,
}

#[derive(Debug, Clone)]
struct Crate {
    contents: char,
}

#[derive(PartialEq, Eq)]
enum Parser {
    ParseStacks,
    ParseCommands,
}

fn parse_stack(line: &str, stacks: &mut Vec<VecDeque<Crate>>) {

    line.chars()
        .skip(1)
        .step_by(4)
        .enumerate()
        .filter(|(_, ch)| ch.is_alphabetic() )
        .for_each(|(i, ch)| stacks[i].push_back( Crate { contents: ch }));
}

fn parse_command(line: &str) -> Result<Command, String> {

    let split: Vec<&str> = line.split(" ").collect();

    match split[..] {
        [ "move", count, "from", src, "to", dst ] => {
            Ok(Command {
                num: count.parse().unwrap(),
                source: src.parse::<usize>().unwrap() - 1,
                destination: dst.parse::<usize>().unwrap() - 1,
            })
        }
        _ => Err("Could not parse line".to_string())
    }
}

fn execute_command(cmd: Command, stacks: &mut Vec<VecDeque<Crate>>) {

    let mut cnt = cmd.num;
    loop {
        if cnt == 0 {
            break;
        }
        let cr = stacks[cmd.source as usize].pop_front().unwrap();
        stacks[cmd.destination as usize].push_front(cr);

        cnt = cnt - 1;
    } 
}

pub fn run(file: String) {

    let mut parser = Parser::ParseStacks;
    let mut stacks: Vec<VecDeque<Crate>> = vec![VecDeque::new(); 10];

    if let Ok(lines) = read_lines(&file) {
        for line in lines {
            if let Ok(line) = line {

                if line == "" {
                    parser = Parser::ParseCommands;
                    continue;
                }

                if parser == Parser::ParseStacks {
                    parse_stack(&line, &mut stacks);
                }
                else if parser == Parser::ParseCommands {
                    if let Ok(cmd) = parse_command(&line) {
                        execute_command(cmd, &mut stacks);
                    }
                    //dbg!(&cmd);
                }
            }
        }

        print!("Stack: ");
        for stack in stacks {
            if let Some(cr) = stack.get(0) {
                print!("{}", cr.contents);
            }
        }
        println!("");
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
