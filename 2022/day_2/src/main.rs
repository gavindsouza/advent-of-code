use std::{fs, str::Lines};

fn main() {
    let input_data = fs::read_to_string("./inputs/test.txt").expect("ohno");
    let input_lines = input_data.lines();
    let p1_score = part_one(input_lines.clone());
    let p2_score = part_two(input_lines.clone());

    println!("Score (p1): {}", p1_score);
    println!("Score (p2): {}", p2_score);
}

fn part_one(input_data: Lines) -> i32 {
    let mut score = 0;

    for line in input_data {
        let plays = line.split(" ").collect::<Vec<&str>>();
        let opponent_play = get_opponent_symbol(plays[0]);
        let my_play = get_my_symbol(plays[1]);

        score += get_score(opponent_play, my_play);
    }
    score
}

fn part_two(input_data: Lines) -> i32 {
    let mut score = 0;

    for line in input_data {
        let plays = line.split(" ").collect::<Vec<&str>>();
        let opponent_play = get_opponent_symbol(plays[0]);
        let required_outcome = get_outcome_symbol(plays[1]);
        let my_play = get_my_play(opponent_play, required_outcome);

        score += get_score(opponent_play, my_play);
    }
    score
}

fn get_my_play<'a>(opponent_play: &'a str, required_outcome: &str) -> &'a str {
    if required_outcome == "draw" {
        return &opponent_play;
    }
    match (opponent_play, required_outcome) {
        ("rock", "win") => "paper",
        ("rock", "lose") => "scissors",
        ("paper", "win") => "scissors",
        ("paper", "lose") => "rock",
        ("scissors", "win") => "rock",
        ("scissors", "lose") => "paper",
        _ => "error",
    }
}

fn get_score(opponent: &str, mine: &str) -> i32 {
    get_play_score(opponent, mine) + get_base_score(mine)
}

fn get_play_score(opponent: &str, mine: &str) -> i32 {
    if opponent == mine {
        return 3;
    }
    match (opponent, mine) {
        ("rock", "paper") => 6,
        ("paper", "scissors") => 6,
        ("scissors", "rock") => 6,
        ("rock", "scissors") => 0,
        ("paper", "rock") => 0,
        ("scissors", "paper") => 0,
        _ => 0,
    }
}

fn get_base_score(play: &str) -> i32 {
    match play {
        "rock" => 1,
        "paper" => 2,
        "scissors" => 3,
        _ => 0,
    }
}

fn get_opponent_symbol(play: &str) -> &str {
    match play {
        "A" => "rock",
        "B" => "paper",
        "C" => "scissors",
        _ => "",
    }
}

fn get_my_symbol(play: &str) -> &str {
    match play {
        "X" => "rock",
        "Y" => "paper",
        "Z" => "scissors",
        _ => "",
    }
}

fn get_outcome_symbol(play: &str) -> &str {
    match play {
        "X" => "lose",
        "Y" => "draw",
        "Z" => "win",
        _ => "",
    }
}
