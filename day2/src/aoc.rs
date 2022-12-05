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


#[derive(Debug, PartialEq, Clone, Copy)]
enum GameMove
{
    Rock,
    Paper,
    Scissors,
    UNKNOWN,
}

#[derive(Debug, PartialEq)]
enum GameResult
{
    Win,
    Loss,
    Draw
}


fn calc_my_move(mine: &GameResult, opp: &GameMove) -> GameMove
{
    if *mine == GameResult::Draw {
        *opp
    }
    else if *mine == GameResult::Win {
        match *opp {
            GameMove::Rock => GameMove::Paper,
            GameMove::Paper => GameMove::Scissors,
            GameMove::Scissors => GameMove::Rock,
            GameMove::UNKNOWN => GameMove::Rock,
        }
    }
    else {
        match *opp {
            GameMove::Rock => GameMove::Scissors,
            GameMove::Paper => GameMove::Rock,
            GameMove::Scissors => GameMove::Paper,
            GameMove::UNKNOWN => GameMove::Rock,
        }
    }
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

fn conv_mine_move(m: &str) -> GameResult 
{
    if m == "Y" {
        GameResult::Draw 
    }
    else if m == "X" {
        GameResult::Loss 
    }
    else if m == "Z" {
        GameResult::Win
    }
    else {
        GameResult::Draw // should never happen with controlled inputs
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
                let my_move = calc_my_move(&mine, &opp);

                let res = calc_game(&my_move, &opp);
                //dbg!(&res);

                let mut score = match my_move {
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
