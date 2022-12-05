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
enum GameMove
{
    Rock,
    Paper,
    Scissors,
    UNKNOWN,
}

#[derive(Debug)]
enum GameResult
{
    Win,
    Loss,
    Draw
}


fn calc_game(mine: &GameMove, opp: &GameMove) -> GameResult
{
    if mine == opp {
        GameResult::Draw
    }
    else if *mine == GameMove::Rock && *opp == GameMove::Scissors {
        GameResult::Win
    }
    else if *mine == GameMove::Scissors && *opp == GameMove::Paper {
        GameResult::Win
    }
    else if *mine == GameMove::Paper && *opp == GameMove::Rock {
        GameResult::Win
    }
    else {
        GameResult::Loss
    }
}

fn conv_opponent_move(m: &str) -> GameMove
{
    if m == "A" {
        GameMove::Rock
    }
    else if m == "B" {
        GameMove::Paper
    }
    else if m == "C" {
        GameMove::Scissors
    }
    else {
        GameMove::UNKNOWN
    }
}

fn conv_mine_move(m: &str) -> GameMove
{
    if m == "Y" {
        GameMove::Paper
    }
    else if m == "X" {
        GameMove::Rock
    }
    else if m == "Z" {
        GameMove::Scissors
    }
    else {
        GameMove::UNKNOWN
    }
}


pub fn run(file: String) {
    if let Ok(lines) = read_lines(&file) {
        let mut total_score = 0;

        for line in lines {
            if let Ok(line) = line {
                let mut split = line.split(" ");

                let move1 = split.next().unwrap();
                let move2 = split.next().unwrap();

                let opp = conv_opponent_move(move1);
                let mine = conv_mine_move(move2);

                //dbg!(&opp);
                //dbg!(&mine);

                let res = calc_game(&mine, &opp);
                //dbg!(&res);

                let mut score = match mine {
                    GameMove::Rock => 1,
                    GameMove::Paper => 2,
                    GameMove::Scissors => 3,
                    GameMove::UNKNOWN => 0,
                };
                score = score + match res {
                    GameResult::Win => 6,
                    GameResult::Draw => 3,
                    GameResult::Loss => 0,
                };
                //dbg!(&score);

                total_score = total_score + score;
            }
        }

        println!("Total score: {}", total_score);
    } else {
        println!("Could not open/read file: {}", &file);
    }
}
