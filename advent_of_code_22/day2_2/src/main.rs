use std::error::Error;
use std::fs;

enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Tie,
    Win,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => panic!("Wrong move"),
        }
    }
}

impl From<char> for Outcome {
    fn from(value: char) -> Self {
        match value {
            'X' => Outcome::Lose,
            'Y' => Outcome::Tie,
            'Z' => Outcome::Win,
            _ => panic!("Wrong action"),
        }
    }
}

fn get_move(opponent_move: Move, desired_outcome: Outcome) -> Move {
    match (opponent_move, desired_outcome) {
        (Move::Rock, Outcome::Lose) => Move::Scissors,
        (Move::Rock, Outcome::Tie) => Move::Rock,
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Paper, Outcome::Lose) => Move::Rock,
        (Move::Paper, Outcome::Tie) => Move::Paper,
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Scissors, Outcome::Lose) => Move::Paper,
        (Move::Scissors, Outcome::Tie) => Move::Scissors,
        (Move::Scissors, Outcome::Win) => Move::Rock,
    }
}

fn score_move(r#move: Move) -> isize {
    match r#move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn score_outcome(outcome: Outcome) -> isize {
    match outcome {
        Outcome::Lose => 0,
        Outcome::Tie => 3,
        Outcome::Win => 6,
    }
}

fn score(opponent_move: Move, desired_outcome: Outcome) -> isize {
    let my_move = get_move(opponent_move, desired_outcome);
    return score_move(my_move) + score_outcome(desired_outcome);
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    let sum: isize = file
        .split("\n")
        .filter(|&line| !line.is_empty())
        .map(|line| {
            assert!(line.len() >= 3);
            let opponent_move: Move = line.chars().nth(0).unwrap().into();
            let desired_outcome: Outcome = line.chars().nth(2).unwrap().into();
            score(opponent_move, desired_outcome)
        })
        .sum();

    println!("Score! {}", sum);

    Ok(())
}
