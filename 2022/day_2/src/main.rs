use std::{fs, str::Lines};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissors,
    Error,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Win,
    Lose,
    Draw,
    Invalid,
}

fn main() {
    let input_data = fs::read_to_string("./inputs/test.txt").expect("ohno");
    let input_lines = input_data.lines();
    let p1 = part_one(input_lines.clone());
    let p2 = part_two(input_lines.clone());

    println!("P1: {}", p1);
    println!("P2: {}", p2);
}

fn part_one(input_data: Lines) -> i32 {
    input_data
        .map(|line| {
            let plays: Vec<&str> = line.split(' ').collect();
            let opponent_play = get_opponent_symbol(plays[0]);
            let my_play = get_my_symbol(plays[1]);
            get_score(opponent_play, my_play)
        })
        .sum()
}

fn part_two(input_data: Lines) -> i32 {
    input_data
        .map(|line| {
            let plays: Vec<&str> = line.split(' ').collect();
            let opponent_play = get_opponent_symbol(plays[0]);
            let required_outcome = get_outcome_symbol(plays[1]);
            let my_play = get_my_play(opponent_play, required_outcome);
            get_score(opponent_play, my_play)
        })
        .sum()
}

fn get_my_play(opponent_play: Play, required_outcome: State) -> Play {
    if required_outcome == State::Draw {
        return opponent_play;
    }
    match (opponent_play, required_outcome) {
        (Play::Rock, State::Win) => Play::Paper,
        (Play::Rock, State::Lose) => Play::Scissors,
        (Play::Paper, State::Win) => Play::Scissors,
        (Play::Paper, State::Lose) => Play::Rock,
        (Play::Scissors, State::Win) => Play::Rock,
        (Play::Scissors, State::Lose) => Play::Paper,
        _ => Play::Error,
    }
}

fn get_score(opponent: Play, mine: Play) -> i32 {
    get_play_score(opponent, mine) + get_base_score(mine)
}

fn get_play_score(opponent: Play, mine: Play) -> i32 {
    if opponent == mine {
        return 3;
    }
    match (opponent, mine) {
        (Play::Rock, Play::Paper) => 6,
        (Play::Paper, Play::Scissors) => 6,
        (Play::Scissors, Play::Rock) => 6,
        (Play::Rock, Play::Scissors) => 0,
        (Play::Paper, Play::Rock) => 0,
        (Play::Scissors, Play::Paper) => 0,
        _ => 0,
    }
}

fn get_base_score(play: Play) -> i32 {
    match play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
        _ => 0,
    }
}

fn get_opponent_symbol(play: &str) -> Play {
    match play {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => Play::Error,
    }
}

fn get_my_symbol(play: &str) -> Play {
    match play {
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissors,
        _ => Play::Error,
    }
}

fn get_outcome_symbol(play: &str) -> State {
    match play {
        "X" => State::Lose,
        "Y" => State::Draw,
        "Z" => State::Win,
        _ => State::Invalid,
    }
}
