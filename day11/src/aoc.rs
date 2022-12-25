use std::fs::File;
use std::io::{self, prelude::*};

fn read_file(filename: &String) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

#[derive(Debug, Copy, Clone)]
struct Test {
    divisor: u64,
    true_condition: u8,
    false_condition: u8,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operator {
    Sum,
    Multiply,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operand {
    Old,
    Constant(u64),
}

impl Operand {
    fn new(input: &str) -> Operand {
        if input == "old" {
            Operand::Old
        } else {
            Operand::Constant(input.parse().unwrap())
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    inspected: u64,
}

use nom::Finish;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, multispace0, one_of, space0, u64, u8},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, (_, _, _, _)) = tuple((tag("Monkey "), u8, tag(":"), line_ending))(i)?;

    let (i, (_, _, items, _)) = tuple((
        space0,
        tag("Starting items: "),
        separated_list1(tag(", "), u64),
        line_ending,
    ))(i)?;

    let (i, (_, _, operator, _, operand, _)) = tuple((
        space0,
        tag("Operation: new = old "),
        one_of("+*/"),
        space0,
        alphanumeric1,
        line_ending,
    ))(i)?;

    let (i, (_, _, divisor, _)) = tuple((space0, tag("Test: divisible by "), u64, line_ending))(i)?;

    let (i, (_, _, true_condition, _)) =
        tuple((space0, tag("If true: throw to monkey "), u8, line_ending))(i)?;

    let (i, (_, _, false_condition, _)) =
        tuple((space0, tag("If false: throw to monkey "), u8, line_ending))(i)?;

    Ok((
        i,
        Monkey {
            items,
            operation: Operation {
                operator: match operator {
                    '+' => Operator::Sum,
                    '*' => Operator::Multiply,
                    _ => unreachable!(),
                },
                operand: Operand::new(operand),
            },
            test: Test {
                divisor,
                true_condition,
                false_condition,
            },
            inspected: 0,
        },
    ))
}

fn parse_monkeys(i: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(multispace0, parse_monkey)(i)
}

fn calculate(operation: &Operation, old: u64) -> u64 {
    let operand2 = match operation.operand {
        Operand::Old => old,
        Operand::Constant(x) => x,
    };

    match operation.operator {
        Operator::Sum => old + operand2,
        Operator::Multiply => old * operand2,
    }
}

fn round(monkeys: &mut Vec<Monkey>) {

    let common_divisor: u64 = monkeys
        .iter()
        .map(|m| m.test.divisor )
        .product();

    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i].clone();

        monkeys[i].inspected += monkey.items.len() as u64;

        for mut item in monkey.items.iter().copied() {
            item = calculate(&monkey.operation, item) % common_divisor;

            let idx = if item % monkey.test.divisor == 0 {
                monkey.test.true_condition as usize
            }
            else {
                monkey.test.false_condition as usize
            };
            let recv = &mut monkeys[idx];

            recv.items.push(item);
        }

        monkeys[i].items.clear();
    }
}

pub fn run(file: String) {
    if let Ok(i) = read_file(&file) {
        let mut monkeys = parse_monkeys(&i).finish().unwrap().1;

        for _ in 0..10000 {
            round(&mut monkeys);
        }

        let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<u64>>();

        inspected.sort_unstable();
        let most_active: u64 = inspected.iter().rev().take(2).product();
        println!("Most active monkeys: {}", most_active);
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
